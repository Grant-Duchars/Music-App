use super::Track;

impl Track {
    pub fn new<T: ToString>(title: T, artist: T, number: usize, length: usize) -> Self {
        Self {
            title: title.to_string(),
            artist: artist.to_string(),
            number,
            length,
        }
    }
}
