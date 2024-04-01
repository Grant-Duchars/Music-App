mod album;
mod song;

use std::rc::Rc;

/// Struct for interfacing with the static albums list.
/// The list is initially uninitialized and so you must
/// use [`Albums::set()`] before using any of its other methods.
pub struct Albums;

pub struct Album {
    pub cover: String,
    pub title: String,
    pub artist: String,
    pub songs: Rc<[Song]>,
}

pub struct Song {
    pub title: String,
    pub artist: String,
    pub number: usize,
}
