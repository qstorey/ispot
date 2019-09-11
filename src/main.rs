use clap::{App, AppSettings, Arg, SubCommand};
use ispot::command;

fn main() {
    let matches = App::new("iTunes to Spotify")
        .version("0.1.0")
        .about("Convert an iTunes playlist to a Spotify playlist")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("itunes")
                .about("Manage iTunes playlists")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("show-playlist")
                        .about("Display an iTunes playlist")
                        .arg(
                            Arg::with_name("playlist")
                                .help("Path to iTunes playlist file")
                                .index(1)
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("spotify")
                .about("Spotify commands")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg(
                    Arg::with_name("SPOTIFY_CLIENT_ID")
                        .env("SPOTIFY_CLIENT_ID")
                        .required(true),
                )
                .arg(
                    Arg::with_name("SPOTIFY_CLIENT_SECRET")
                        .env("SPOTIFY_CLIENT_SECRET")
                        .required(true),
                )
                .subcommand(
                    SubCommand::with_name("auth").about("Authenticate with the Spotify API"),
                )
                .subcommand(
                    SubCommand::with_name("create-playlist")
                        .about("Create an empty Spotify playist.")
                        .arg(
                            Arg::with_name("name")
                                .help("Name of the Spotify playlist.")
                                .index(1)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("list-playlists").about("List Spotify playlists."),
                )
                .subcommand(
                    SubCommand::with_name("match-track")
                        .about("Match a track with Spotify.")
                        .arg(
                            Arg::with_name("name")
                                .help("Name of the track")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("artist")
                                .help("Name of the artist who appeared on track")
                                .index(2)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("album")
                                .help("Name of the album the track appeared on")
                                .index(3)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("match-playlist")
                        .about("Match an iTunes playlists with tracks on Spotify.")
                        .arg(
                            Arg::with_name("playlist")
                                .help("Path to iTunes playlist file")
                                .required(true),
                        )
                        .arg(Arg::with_name("print-only")
                             .help("Only print the matched playlist, don't create the Spotify playlist")
                             .long("print-only")
                        )
                        .arg(Arg::with_name("playlist-name")
                             .help("Creates a Spotify playlist with the specified name. If this is not provided a playlist name is automatically generated")
                             .long("playlist-name")
                             .takes_value(true)
                             .conflicts_with("print-only")
                        )


                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("itunes", Some(itunes_matches)) => match itunes_matches.subcommand() {
            ("show-playlist", Some(show_playlist_matches)) => {
                command::show_playlist(show_playlist_matches)
            }
            _ => unreachable!(),
        },
        ("spotify", Some(spotify_matches)) => {
            let spotify_client_id = spotify_matches.value_of("SPOTIFY_CLIENT_ID").unwrap();
            let spotify_client_secret = spotify_matches.value_of("SPOTIFY_CLIENT_SECRET").unwrap();

            match spotify_matches.subcommand() {
                ("auth", Some(_)) => {
                    command::authenticate(&spotify_client_id, &spotify_client_secret)
                }
                ("create-playlist", Some(spotify_create_playlist_matches)) => {
                    command::create_playlist(
                        &spotify_client_id,
                        &spotify_client_secret,
                        spotify_create_playlist_matches,
                    )
                }
                ("list-playlists", Some(_)) => {
                    command::list_playlists(&spotify_client_id, &spotify_client_secret)
                }
                ("match-playlist", Some(spotify_match_playlist_matches)) => {
                    command::match_playlist(
                        &spotify_client_id,
                        &spotify_client_secret,
                        spotify_match_playlist_matches,
                    )
                }
                ("match-track", Some(spotify_match_track_matches)) => command::match_track(
                    &spotify_client_id,
                    &spotify_client_secret,
                    spotify_match_track_matches,
                ),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
