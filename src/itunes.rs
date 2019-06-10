use crate::error::{Error, ErrorKind};
use crate::types;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

pub fn load_playlist(path: &str) -> Result<Playlist, Error> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(Error::new(ErrorKind::FileNotFound(
            "Can't find playlist file".to_string(),
        )));
    }
    if !p.is_file() {
        return Err(Error::new(ErrorKind::NotAFile(
            "Expected a playlist file, found a directory".to_string(),
        )));
    }

    let playlist: Playlist = plist::from_file(&path)?;
    Ok(playlist)
}

impl From<plist::Error> for Error {
    fn from(_: plist::Error) -> Self {
        // TODO: Get actual error instead of hard coding a placeholder.
        Error::new(ErrorKind::PlistError("Failed to load plist".to_string()))
    }
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
    #[serde(rename = "Major Version")]
    major_version: i32,
    #[serde(rename = "Music Folder")]
    pub music_folder: String,
    #[serde(rename = "Tracks")]
    pub tracks: BTreeMap<String, Track>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Artist")]
    pub artist: String,
    #[serde(rename = "Album")]
    pub album: Option<String>,
    #[serde(rename = "Year")]
    pub year: Option<i16>,
    #[serde(rename = "Genre")]
    pub genre: Option<String>,
}

impl types::DisplayTrack for Track {
    fn name(&self) -> &str {
        &self.name
    }

    fn artist(&self) -> &str {
        &self.artist
    }

    fn album(&self) -> Option<&String> {
        self.album.as_ref()
    }

    fn spotify_uri(&self) -> &str {
        "n/a"
    }
}
