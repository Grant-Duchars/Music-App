use super::Song;

impl Song {
    pub fn new<T: ToString>(title: T, artist: T, number: usize) -> Self {
        Song {
            title: title.to_string(),
            artist: artist.to_string(),
            number,
        }
    }
}
