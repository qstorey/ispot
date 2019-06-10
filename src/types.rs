pub trait DisplayTrack {
    fn name(&self) -> &str;
    fn artist(&self) -> &str;
    // TODO: Revisit this. Option<&String> might not be the best idea.
    fn album(&self) -> Option<&String>;
    fn spotify_uri(&self) -> &str;
}
