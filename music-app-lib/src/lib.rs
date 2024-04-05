use regex::Regex;
use std::fs;

mod album;
mod song;

/// Struct for interfacing with the static albums list.
/// The list is initially uninitialized and so you must
/// use [`Albums::set()`] before using any of its other methods.
pub struct Albums;

pub struct Album {
    pub cover: String,
    pub title: String,
    pub artist: String,
    pub songs: std::rc::Rc<[Song]>,
    pub runtime: usize,
}

pub struct Song {
    pub title: String,
    pub artist: String,
    pub number: usize,
    pub length: usize,
}

/// Function to mock pulling music data from backend
#[allow(unused)]
fn new_get_albums() -> Vec<Album> {
    let mut albums = Vec::new();
    let music_library = "./mock-music-data/";
    let regex = Regex::new(r"cover\.(jpe?g|png)").expect("valid regex");
    for artist in fs::read_dir(music_library).unwrap() {
        let artist = artist.unwrap();
        let mut songs = Vec::<Song>::new();
        for (index, song) in fs::read_dir(artist.path()).unwrap().enumerate() {
            let song = song.unwrap();
            if song.file_name() == "cover.png" {
                continue;
            }
            let file = fs::read_to_string(song.path()).unwrap();
            println!("{file}");
        }
    }
    albums
}

use rand::Rng;

/// Function to mock pulling music data from backend
pub fn get_albums() -> Vec<Album> {
    let mut rng = rand::thread_rng();
    let mut runtimes = (0..270).map(|_| rng.gen_range(40..250_usize));
    let artist = "Doseone";
    let mut titles = [
        "Boot Up",
        "Glug Sucks",
        "Climbing Pipe",
        "Mosca Mosca Mosca",
        "Nothing Works",
        "Island Fever",
        "Tia Knows Best",
        "All Dusk Everything",
        "Hi Frog Eye",
        "B Boys Revenge",
        "Ll Cool Hans",
        "Bass Water",
        "Sedative Express",
        "Double Double",
        "The Whistler",
        "Big Mud Dont Sleep",
        "Ghost Phone Home",
        "Uzzi Does It",
        "Dryer Trier",
        "Zoomed Out",
        "Burgermon",
        "Toxic Environment",
        "Eyes Out",
        "Ghost Whip the Ceo Ship",
        "Life Loop Blues",
        "Big Mud - Bubble Up",
        "Big Mud - Sludge Life",
    ]
    .into_iter();
    let mut songs = Vec::new();
    for i in 1_usize..=27 {
        songs.push(Song::new(
            titles.next().unwrap(),
            artist,
            i,
            runtimes.next().unwrap(),
        ))
    }
    let songs = std::rc::Rc::<[Song]>::from(songs);
    let mut albums = Vec::with_capacity(10);
    for _ in 0..10 {
        albums.push(Album::new(
            "/public/SludgeLifeOST.png",
            "Sludge Life",
            "Doseone",
            songs.clone(),
        ));
    }
    albums
}

/// Calculates an album or songs' runtime/length from a given number \
/// # Examples
/// ```
/// use music_app_lib::calc_runtime;
/// assert_eq!([0, 0, 1], calc_runtime(1));
/// assert_eq!([0, 1, 1], calc_runtime(61));
/// assert_eq!([1, 1, 1], calc_runtime(3_661));
/// assert_eq!([0, 0, 59], calc_runtime(59));
/// assert_eq!([0, 59, 59], calc_runtime(3_599));
/// assert_eq!([999, 59, 59], calc_runtime(3_599_999));
/// ```
pub fn calc_runtime(num: usize) -> [usize; 3] {
    const HOUR: usize = 3600;
    const MINUTE: usize = 60;
    let hours = num / HOUR;
    let left = num % HOUR;
    let minutes = left / MINUTE;
    let seconds = left % MINUTE;
    [hours, minutes, seconds]
}

/// Converts a number to a string based on how many seconds it represents \
/// # Examples
/// ```
/// use music_app_lib::to_words;
/// // Formats both non-plural ...
/// assert_eq!("1 sec", to_words(1));
/// assert_eq!("1 min, 1 sec", to_words(61));
/// assert_eq!("1 hour, 1 min, 1 sec", to_words(3_661));
/// // ... and plural quantities.
/// assert_eq!("59 secs", to_words(59));
/// assert_eq!("59 mins, 59 secs", to_words(3_599));
/// assert_eq!("999 hours, 59 mins, 59 secs", to_words(3_599_999));
///
/// ```
pub fn to_words(num: usize) -> String {
    let runtime = calc_runtime(num);
    let [h_plural, m_plural, s_plural] = runtime.map(|n| if n != 1 { "s" } else { "" });
    match runtime {
        [0, 0, sec] => {
            format!("{sec} sec{s_plural}")
        }
        [0, min, sec] => {
            format!("{min} min{m_plural}, {sec} sec{s_plural}")
        }
        [hour, min, sec] => {
            format!("{hour} hour{h_plural}, {min} min{m_plural}, {sec} sec{s_plural}")
        }
    }
}

/// Converts a number to a string based on how many seconds it represents \
/// # Examples
/// ```
/// use music_app_lib::to_digital;
/// assert_eq!("0:01", to_digital(1));
/// assert_eq!("1:01", to_digital(61));
/// assert_eq!("1:01:01", to_digital(3_661));
/// assert_eq!("0:59", to_digital(59));
/// assert_eq!("59:59", to_digital(3_599));
/// assert_eq!("999:59:59", to_digital(3_599_999));
/// ```
pub fn to_digital(num: usize) -> String {
    match calc_runtime(num) {
        [0, 0, sec] => {
            format!("0:{sec:0>2}")
        }
        [0, min, sec] => {
            format!("{min}:{sec:0>2}")
        }
        [hour, min, sec] => {
            format!("{hour}:{min:0>2}:{sec:0>2}")
        }
    }
}
