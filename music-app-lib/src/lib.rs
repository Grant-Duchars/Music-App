mod album;
pub mod mocking;
pub mod runtime;
mod track;

use reactive_stores::{Patch, Store};

#[derive(Debug, Store, Patch)]
pub struct Albums {
    albums: Vec<Album>,
}

#[derive(Debug, Store, Patch)]
pub struct Album {
    cover: String,
    title: String,
    /// Artist of the album, songs in the album may have other artists
    artist: String,
    #[store(key: usize = |track| track.number)]
    tracks: Vec<Track>,
    genre: String,
    /// Computed from track list.
    runtime: usize,
}

#[derive(Debug, Store, Patch)]
pub struct Track {
    title: String,
    artist: String,
    /// The track number on the track's album
    number: usize,
    /// The duration of the track in seconds
    length: usize,
}
