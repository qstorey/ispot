# ispot

Convert iTunes playlists to Spotify playlists.

> WARNING: This project is a work in progress.

## Installation

Register your Spotify application in the developer [dashboard](https://developer.spotify.com/documentation/general/guides/app-settings/).

```bash
cargo install ispot
```

## Usage

```
# Authenticate with the Spotify API
ispot auth

# Match iTunes playlist and create Spotify playlist
ispot spotify match-playlist /path/to/itunes/playlist

# More help
ispot --help
```

## Development

```bash
# Clone the repo
git clone https://github.com/qstorey/ispot

# Build a debug version
cd ispot
cargo build

# Run the unit tests
cargo test
```
