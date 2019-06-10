use crate::types::DisplayTrack;
use prettytable::{format, Cell, Row, Table};

pub fn tabulate_track<T: DisplayTrack>(track: &T) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![Cell::new("Name"), Cell::new(&track.name())]));
    table.add_row(Row::new(vec![
        Cell::new("Arist"),
        Cell::new(&track.artist()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Album"),
        Cell::new(&track.album().unwrap_or(&"".to_string())),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Spotify URI"),
        Cell::new(&track.spotify_uri()),
    ]));

    table.printstd();
}

pub fn tabulate_tracks<T: DisplayTrack>(tracks: Vec<T>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.set_titles(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Artist"),
        Cell::new("Album"),
        Cell::new("Spotify URI"),
    ]));

    for track in &tracks {
        table.add_row(Row::new(vec![
            Cell::new(&track.name()),
            Cell::new(&track.artist()),
            Cell::new(&track.album().unwrap_or(&"".to_string())),
            Cell::new(&track.spotify_uri()),
        ]));
    }

    table.printstd();
}
