use super::{Album, Albums, Track};

impl Albums {
    /// Creates a new `Albums` struct
    pub fn new(albums: Vec<Album>) -> Self {
        Self { albums }
    }

    pub fn check(&self, index: usize) -> bool {
        index < self.albums.len()
    }

    pub fn len(&self) -> usize {
        self.albums.len()
    }

    pub fn is_empty(&self) -> bool {
        self.albums.is_empty()
    }

    /// Gets the total number of tracks among albums in the albums list
    pub fn total_tracks(&self) -> usize {
        self.albums.iter().map(|a| a.tracks.len()).sum()
    }

    /// Get the total runtime of the albums list
    pub fn runtime(&self) -> usize {
        self.albums.iter().map(|a| a.runtime).sum()
    }
}

impl Album {
    /// Creates an `Album`
    pub fn new<T: ToString>(cover: T, title: T, artist: T, tracks: Vec<Track>, genre: T) -> Self {
        let runtime = Self::collect_runtime(&tracks);
        Album {
            cover: cover.to_string(),
            title: title.to_string(),
            artist: artist.to_string(),
            tracks,
            genre: genre.to_string(),
            runtime,
        }
    }

    fn collect_runtime(songs: &[Track]) -> usize {
        songs.iter().fold(0, |acc, s| acc + s.length)
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.artist == other.artist
            && self.genre == other.genre
            && self.runtime == other.runtime
    }
}
