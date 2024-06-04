//! This library is used to sync structs between the front and backend and so I can use doctests on certain functions

/// Module that includes impls for [`Albums`] and [`Album`]
mod album;
/// Module that includes functions for mocking album data
pub mod mocking;
/// Module that includes functions for formatting runtimes
pub mod runtime;
/// Module that includes impls for [`Song`]
mod song;

/// Struct for interfacing with the static albums list.
/// The list is initially uninitialized and so you must
/// use [`Albums::set()`] before using any of its other methods.
pub struct Albums;

/// Struct for storing data about an album
#[derive(Debug)]
pub struct Album {
    /// Url? to cover.jpg
    pub cover: String,
    /// Title of the album
    pub title: String,
    /// Artist of the album, songs in the album
    /// may have other artists
    pub artist: String,
    /// Sorted list of [`Song`]s
    pub songs: Box<[Song]>,
    /// Genre of the album
    pub genre: String,
    /// Computed from songs list \
    /// Runtime of the album
    pub runtime: usize,
}

/// Struct for storing data about a song
#[derive(Debug)]
pub struct Song {
    /// The title of the song
    pub title: String,
    /// The artist of the song
    pub artist: String,
    /// The track number on the song's album
    pub number: usize,
    /// The duration of the song in seconds
    pub length: usize,
}
