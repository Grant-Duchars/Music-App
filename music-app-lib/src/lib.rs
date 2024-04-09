/// ! This library is used to sync structs between the front and backend and so I can use doctests on certain functions

/// Module that includes impls for [`Albums`] and [`Album`]
mod album;
/// Module that inclues impls for [`Song`]
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

/// Function to mock pulling music data from backend
pub fn get_albums() -> Vec<Album> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut runtimes = (0..297).map(|_| rng.gen_range(40..250_usize));
    let titles = [
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
    let mut albums = Vec::with_capacity(10);
    for _ in 0..9 {
        let mut songs = Vec::with_capacity(27);
        for (i, title) in titles.clone().enumerate() {
            songs.push(Song::new(title, "Doseone", i, runtimes.next().unwrap()))
        }
        let songs = Box::<[Song]>::from(songs);
        albums.push(Album::new(
            "/public/cover.png",
            "Sludge Life",
            "Doseone",
            songs,
            "Soundtrack",
        ));
    }
    let mut songs = Vec::with_capacity(54);
    for (i, title) in titles.clone().enumerate() {
        songs.push(Song::new(title, "Doseone", i, runtimes.next().unwrap()))
    }
    for (i, title) in titles.clone().enumerate() {
        songs.push(Song::new(title, "Doseone", i, runtimes.next().unwrap()))
    }
    let songs = Box::<[Song]>::from(songs);
    albums.push(Album::new(
        "/public/cover.png",
        "Sludge Life",
        "Doseone",
        songs,
        "Soundtrack",
    ));
    albums
}

/// Calculates an album or songs' runtime/length from a given number \
/// # Examples
/// ```
/// use music_app_lib::calc_runtime;
///
/// assert_eq!([0, 0, 0, 0, 1], calc_runtime(1));
/// assert_eq!([0, 0, 0, 1, 1], calc_runtime(61));
/// assert_eq!([0, 0, 1, 1, 1], calc_runtime(3_661));
/// assert_eq!([0, 1, 1, 1, 1], calc_runtime(90_061));
/// assert_eq!([1, 1, 1, 1, 1], calc_runtime(694_861));
///
/// assert_eq!([0, 0, 0, 0, 59], calc_runtime(59));
/// assert_eq!([0, 0, 0, 59, 59], calc_runtime(3_599));
/// assert_eq!([0, 0, 23, 59, 59], calc_runtime(86_399));
/// assert_eq!([0, 6, 23, 59, 59], calc_runtime(604_799));
/// assert_eq!([999, 6, 23, 59, 59], calc_runtime(604_799_999));
/// ```
pub fn calc_runtime(num: usize) -> [usize; 5] {
    const WEEK: usize = 604800;
    const DAY: usize = 86400;
    const HOUR: usize = 3600;
    const MINUTE: usize = 60;
    let (weeks, left) = (num / WEEK, num % WEEK);
    let (days, left) = (left / DAY, left % DAY);
    let (hours, left) = (left / HOUR, left % HOUR);
    let (minutes, seconds) = (left / MINUTE, left % MINUTE);
    [weeks, days, hours, minutes, seconds]
}

#[test]
fn test_runtime() {
    println!("{:#?}", calc_runtime(4_294_967_295));
}

/// Converts a number to a string based on how many seconds it represents \
/// # Examples
/// ```
/// use music_app_lib::to_words;
/// // Formats both non-plural ...
/// assert_eq!("1 sec", to_words(1));
/// assert_eq!("1 min, 1 sec", to_words(61));
/// assert_eq!("1 hour, 1 min, 1 sec", to_words(3_661));
/// assert_eq!("1 day, 1 hour, 1 min, 1 sec", to_words(90_061));
/// assert_eq!("1 week, 1 day, 1 hour, 1 min, 1 sec", to_words(694_861));
/// // ... and plural quantities.
/// assert_eq!("59 secs", to_words(59));
/// assert_eq!("59 mins, 59 secs", to_words(3_599));
/// assert_eq!("23 hours, 59 mins, 59 secs", to_words(86_399));
/// assert_eq!("6 days, 23 hours, 59 mins, 59 secs", to_words(604_799));
/// assert_eq!("999 weeks, 6 days, 23 hours, 59 mins, 59 secs", to_words(604_799_999));
/// ```
pub fn to_words(num: usize) -> String {
    let runtime = calc_runtime(num);
    let [w_plural, d_plural, h_plural, m_plural, s_plural] =
        runtime.map(|n| if n != 1 { "s" } else { "" });
    match runtime {
        [0, 0, 0, 0, sec] => {
            format!("{sec} sec{s_plural}")
        }
        [0, 0, 0, min, sec] => {
            format!("{min} min{m_plural}, {sec} sec{s_plural}")
        }
        [0, 0, hour, min, sec] => {
            format!("{hour} hour{h_plural}, {min} min{m_plural}, {sec} sec{s_plural}")
        }
        [0, day, hour, min, sec] => {
            format!("{day} day{d_plural}, {hour} hour{h_plural}, {min} min{m_plural}, {sec} sec{s_plural}")
        }
        [week, day, hour, min, sec] => {
            format!("{week} week{w_plural}, {day} day{d_plural}, {hour} hour{h_plural}, {min} min{m_plural}, {sec} sec{s_plural}")
        }
    }
}

/// Converts a number to a string based on how many seconds it represents \
/// # Examples
/// ```
/// use music_app_lib::to_digital;
///
/// assert_eq!("0:01", to_digital(1));
/// assert_eq!("1:01", to_digital(61));
/// assert_eq!("1:01:01", to_digital(3_661));
/// assert_eq!("1:01:01:01", to_digital(90_061));
/// assert_eq!("1:01:01:01:01", to_digital(694_861));
///
/// assert_eq!("0:59", to_digital(59));
/// assert_eq!("59:59", to_digital(3_599));
/// assert_eq!("23:59:59", to_digital(86_399));
/// assert_eq!("6:23:59:59", to_digital(604_799));
/// assert_eq!("999:06:23:59:59", to_digital(604_799_999));
/// ```
pub fn to_digital(num: usize) -> String {
    match calc_runtime(num) {
        [0, 0, 0, 0, sec] => {
            format!("0:{sec:0>2}")
        }
        [0, 0, 0, min, sec] => {
            format!("{min}:{sec:0>2}")
        }
        [0, 0, hour, min, sec] => {
            format!("{hour}:{min:0>2}:{sec:0>2}")
        }
        [0, day, hour, min, sec] => {
            format!("{day}:{hour:0>2}:{min:0>2}:{sec:0>2}")
        }
        [week, day, hour, min, sec] => {
            format!("{week}:{day:0>2}:{hour:0>2}:{min:0>2}:{sec:0>2}")
        }
    }
}
