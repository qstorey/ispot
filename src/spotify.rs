use crate::error::{Error, ErrorKind};
use crate::types;
use failure;
use rspotify::spotify::client::{ApiError, Spotify};
use rspotify::spotify::model::playlist::{FullPlaylist, SimplifiedPlaylist};
use rspotify::spotify::model::track::FullTrack;
use rspotify::spotify::oauth2::SpotifyClientCredentials;
use rspotify::spotify::oauth2::SpotifyOAuth;
use rspotify::spotify::oauth2::TokenInfo;
use rspotify::spotify::util::get_token;

const SPOTIFY_CLIENT_REDIRECT_URI: &str = "http://localhost:8080/";

/// Authenticate with the Spotify API and retrieve an API token.
pub fn authenticate(
    spotify_client_id: &str,
    spotify_client_secret: &str,
) -> Result<TokenInfo, Error> {
    let mut oauth = SpotifyOAuth::default()
        .client_id(spotify_client_id)
        .client_secret(spotify_client_secret)
        .redirect_uri(&SPOTIFY_CLIENT_REDIRECT_URI)
        .scope("user-read-recently-played playlist-read-private playlist-modify-private")
        .build();

    match get_token(&mut oauth) {
        Some(token_info) => Ok(token_info),
        None => Err(Error::new(ErrorKind::AuthenticationFailure(
            "Failed to get Spotify OAuth token".to_string(),
        ))),
    }
}

pub struct SpotifyWrapper {
    client: Spotify,
}

impl SpotifyWrapper {
    /// Create a new Spotify Client
    pub fn new(
        spotify_client_id: &str,
        spotify_client_secret: &str,
    ) -> Result<SpotifyWrapper, Error> {
        let token = authenticate(&spotify_client_id, &spotify_client_secret)?;
        let credentials = SpotifyClientCredentials::default()
            .token_info(token)
            .build();
        Ok(SpotifyWrapper {
            client: Spotify::default()
                .client_credentials_manager(credentials)
                .build(),
        })
    }

    /// Generate a search query string according to the Spotify
    /// [docs](https://developer.spotify.com/documentation/web-api/reference/search/search/#writing-a-query---guidelines)
    fn generate_search_query(name: &str, artist: Option<&str>, album: Option<&str>) -> String {
        let mut res = name.to_owned();

        //TODO: Cleanup string formatting
        res = match artist {
            Some(ref t) => format!("{} artist:{}", res, t),
            None => res,
        };

        res = match album {
            Some(ref t) => format!("{} album:{}", res, t),
            None => res,
        };

        res
    }

    /// Create a user playlist
    pub fn create_playlist(&self, name: &str) -> Result<FullPlaylist, Error> {
        let user_id: String = self.user_id()?;
        self.rate_limit_call(|spotify| spotify.user_playlist_create(&user_id, name, false, None))
    }

    /// Attempt an exact track match otherwise error no or multiple results are found.
    pub fn exact_track_match(
        &self,
        name: &str,
        artist: Option<&str>,
        album: Option<&str>,
    ) -> Result<FullTrack, Error> {
        let search_query = SpotifyWrapper::generate_search_query(&name, artist, album);

        let result =
            self.rate_limit_call(|spotify| spotify.search_track(&search_query, 1, 0, None))?;
        let page = result.tracks;
        if page.total == 0 {
            return Err(Error::new(ErrorKind::NoResults));
        }
        if page.total > 1 {
            return Err(Error::new(ErrorKind::MultipleResults(page.total)));
        }

        Ok(page.items[0].clone())
    }

    /// List the user's playlists.
    pub fn list_playlists(&self) -> Result<Vec<SimplifiedPlaylist>, Error> {
        let mut playlists: Vec<SimplifiedPlaylist> = Vec::new();

        let mut offset = 0;
        let limit = 20;
        loop {
            let pagninator =
                self.rate_limit_call(|spotify| spotify.current_user_playlists(limit, offset))?;
            playlists.append(&mut pagninator.items.clone());
            if pagninator.next.is_some() {
                println!("paginating");
                offset += 1;
            } else {
                break;
            }
        }
        Ok(playlists)
    }

    /// Return the user id from the access token
    fn user_id(&self) -> Result<String, Error> {
        Ok(self.rate_limit_call(|spotify| spotify.me())?.id)
    }

    fn rate_limit_call<F, R>(&self, func: F) -> Result<R, Error>
    where
        F: Fn(&Spotify) -> Result<R, failure::Error>,
    {
        match func(&self.client) {
            Ok(v) => Ok(v),
            Err(e) => match e
                .downcast::<ApiError>()
                .expect("unable to handle spotify error")
            {
                ApiError::Unauthorized => Err(Error::new(ErrorKind::Unauthorized)),
                ApiError::RateLimited(d) => {
                    let duration = d.unwrap_or(10);
                    println!("spotify rate limit hit. sleeping for {} seconds", duration);
                    std::thread::sleep(std::time::Duration::from_secs(duration as u64));
                    self.rate_limit_call(func)
                }
                ApiError::Other(status_code) => panic!(
                    "api called to spotify failed with status code {}",
                    status_code
                ),
            },
        }
    }
}

impl types::DisplayTrack for FullTrack {
    fn name(&self) -> &str {
        &self.name
    }

    fn artist(&self) -> String {
        let mut result = String::new();
        for artist in &self.artists {
            result.push_str(&artist.name);
            result.push_str("; ");
        }
        result
    }

    fn album(&self) -> Option<&String> {
        Some(&self.album.name)
    }

    fn spotify_uri(&self) -> &str {
        &self.uri
    }
}

#[cfg(test)]
mod tests {
    use super::SpotifyWrapper;

    #[test]
    fn test_generate_search_query() {
        assert_eq!(
            SpotifyWrapper::generate_search_query("Bohemian Rhapsody", None, None),
            "Bohemian Rhapsody"
        );
        assert_eq!(
            SpotifyWrapper::generate_search_query("Bohemian Rhapsody", Some("Queen"), None),
            "Bohemian Rhapsody artist:Queen"
        );
        assert_eq!(
            SpotifyWrapper::generate_search_query(
                "Bohemian Rhapsody",
                Some("Queen"),
                Some("A Night at the Opera")
            ),
            "Bohemian Rhapsody artist:Queen album:A Night at the Opera"
        );
        assert_eq!(
            SpotifyWrapper::generate_search_query(
                "Bohemian Rhapsody",
                None,
                Some("A Night at the Opera")
            ),
            "Bohemian Rhapsody album:A Night at the Opera"
        );
    }
}
