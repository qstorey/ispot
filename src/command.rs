use crate::error::ErrorKind;
use crate::itunes;
use crate::output;
use crate::spotify;
use clap::ArgMatches;
use rspotify::spotify::model::track::FullTrack;
use std::process;

pub fn authenticate(spotify_client_id: &str, spotify_client_secret: &str) {
    match spotify::authenticate(&spotify_client_id, &spotify_client_secret) {
        Ok(_) => println!("Successfully authenticated to Spotify API"),
        Err(error) => println!("{}", error),
    }
}

/// Create a Spotify playlist.
pub fn create_playlist(spotify_client_id: &str, spotify_client_secret: &str, matches: &ArgMatches) {
    let name = matches.value_of("name").unwrap();
    let spotify_wrapper =
        spotify::SpotifyWrapper::new(&spotify_client_id, &spotify_client_secret).unwrap();
    let playlist = match spotify_wrapper.create_playlist(&name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    output::tabulate_playlist(&playlist);
}

/// List Spotify playlists.
pub fn list_playlists(spotify_client_id: &str, spotify_client_secret: &str) {
    let spotify_wrapper =
        spotify::SpotifyWrapper::new(&spotify_client_id, &spotify_client_secret).unwrap();
    let playlists = match spotify_wrapper.list_playlists() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    output::tabulate_playlists(&playlists);
}

pub fn match_playlist(spotify_client_id: &str, spotify_client_secret: &str, matches: &ArgMatches) {
    let path_to_playlist = matches.value_of("playlist").unwrap();

    let spotify_wrapper =
        spotify::SpotifyWrapper::new(&spotify_client_id, &spotify_client_secret).unwrap();

    let playlist = match itunes::load_playlist(&path_to_playlist) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let mut match_tracks: Vec<FullTrack> = Vec::new();
    let mut mismatched_tracks: i32 = 0;

    for track in playlist.tracks.values() {
        match spotify_wrapper.exact_track_match(
            &track.name,
            Some(&track.artist),
            track.album.as_ref().map(|x| &**x),
        ) {
            Ok(spotify_track) => match_tracks.push(spotify_track.clone()),
            Err(e) => match e.kind {
                ErrorKind::MultipleResults(_) | ErrorKind::NoResults => mismatched_tracks += 1,
                _ => panic!("unhandled exception {}", e),
            },
        }
    }

    output::tabulate_tracks(match_tracks);
    println!("mismatched tracks: {}", mismatched_tracks);
    println!("total tracks: {}", playlist.tracks.len());
}

pub fn match_track(spotify_client_id: &str, spotify_client_secret: &str, matches: &ArgMatches) {
    let name = matches.value_of("name").unwrap();
    let artist = matches.value_of("artist").unwrap();
    let album = matches.value_of("album").unwrap();

    let spotify_wrapper =
        spotify::SpotifyWrapper::new(&spotify_client_id, &spotify_client_secret).unwrap();
    let track = match spotify_wrapper.exact_track_match(&name, Some(&artist), Some(&album)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    output::tabulate_track(&track);
}

pub fn show_playlist(matches: &ArgMatches) {
    let path_to_playlist = matches.value_of("playlist").unwrap();

    let playlist = match itunes::load_playlist(&path_to_playlist) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    output::tabulate_tracks(playlist.tracks.values().cloned().collect());
}
