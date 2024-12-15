//! This library is used to sync structs between the front and backend and so I can use doctests on certain functions
mod album;
pub mod mocking;
pub mod runtime;
mod track;

use reactive_stores::Store;

#[derive(Store)]
pub struct Albums {
    albums: Vec<Album>,
}

#[derive(Debug, Store)]
pub struct Album {
    cover: String,
    title: String,
    /// Artist of the album, songs in the album may have other artists
    artist: String,
    #[store(key: usize = |track| track.number)]
    tracks: Vec<Track>,
    genre: String,
    /// Computed from track list.
    /// Runtime of the album
    runtime: usize,
}

#[derive(Debug, Store)]
pub struct Track {
    title: String,
    artist: String,
    /// The track number on the track's album
    number: usize,
    /// The duration of the track in seconds
    length: usize,
}
