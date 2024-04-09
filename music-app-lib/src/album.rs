use super::{Album, Albums, Song};
use std::{mem::MaybeUninit, rc::Rc, sync::Once};

static mut ALBUMS: MaybeUninit<Rc<[Album]>> = MaybeUninit::uninit();
static INIT_ALBUMS: Once = Once::new();

impl Albums {
    /// Sets the albums list
    pub fn set(albums: Vec<Album>) {
        unsafe {
            match INIT_ALBUMS.is_completed() {
                false => INIT_ALBUMS.call_once(|| {
                    ALBUMS.write(Rc::from(albums));
                }),
                true => {
                    ALBUMS.assume_init_drop();
                    ALBUMS.write(Rc::from(albums));
                }
            }
        }
    }
    /// Gets a reference to the albums list
    pub fn get() -> Option<&'static [Album]> {
        unsafe {
            match INIT_ALBUMS.is_completed() {
                true => Some(ALBUMS.assume_init_ref()),
                false => None,
            }
        }
    }
    /// Gets a reference to an album at the given index
    pub fn get_album(index: usize) -> Option<&'static Album> {
        Self::get()?.get(index)
    }
    /// Gets the length of the albums list
    pub fn len() -> Option<usize> {
        Some(Self::get()?.len())
    }
}

impl Album {
    /// Creates an `Album`
    pub fn new<T: ToString>(cover: T, title: T, artist: T, songs: Box<[Song]>, genre: T) -> Self {
        let runtime = Self::collect_runtime(&songs);
        Album {
            cover: cover.to_string(),
            title: title.to_string(),
            artist: artist.to_string(),
            songs,
            genre: genre.to_string(),
            runtime,
        }
    }

    fn collect_runtime(songs: &Box<[Song]>) -> usize {
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
