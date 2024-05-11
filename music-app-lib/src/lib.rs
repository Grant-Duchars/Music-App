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
    let mut albums = Vec::new();
    let mut rng = rand::thread_rng();
    let titles = Box::from_iter(
        [
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
        .map(|s| s.into()),
    );
    albums.push(create_album(
        &mut rng,
        "/public/mock-covers/sl.jpg",
        "Sludge Life",
        "Doseone",
        Some(titles),
        None,
    ));
    let titles = Box::from_iter(
        [
            "Bug Fables",
            "The Everlasting Sapling",
            "Where Adverturers Gather",
            "FIGHT!",
            "Chatper 1: A Dysfunctional Trio",
            "Outskirts",
            "Battle Won!",
            "Getting Stronger!",
            "Gameover",
            "Badge Get",
            "Key Item",
            "Snakemouth Den",
            "It's Getting Scary!",
            "His Friends Call Him Spuder (Don't Call Him Spuder)",
            "The One Left Behind (Lief's Theme)",
            "Lost Treasure",
            "Ant Kingdom",
            "In the Court of the Ant Queen",
            "Chapter 2 - Sacred Golden Hills",
            "Night at the Inn",
            "Fry It Up!",
            "Fine Arts",
            "The Sailor's Pier",
            "Oh no! WASPS!!",
            "Caves",
            "Golden Lands",
            "Kut It Up!",
            "Harvest Festival",
            "Team, This One's Stronger",
            "The Sacred Hills",
            "Venus, Godess of Bountiful Harvest",
            "Frenzied Sunflower Dance",
            "Chapter 3 - Factory Inspection",
            "Dodgy Business",
            "Reckless for Glory!",
            "Lost Sands",
            "Defiant Root",
            "Make it Crisbee!",
            "Drums of War",
            "High Above, Bee Kingdon",
            "Bugaria's Latest Sensation!",
            "Bianca, Queen of all Bees",
            "Work That Honey",
            "Store That Honey",
            "MECHA BEE DESTROYER BLASTLORD",
            "Chapter 4 - Mysterious Lost Sands",
            "The Bandit's Hideout",
            "The Ones Who...",
            "Rise from the Sands",
            "Lost Castle of Ancient Worship",
            "The Watcher",
            "Ant Kingdom, Under Attack!",
            "The Usurper",
            "Chapter 5 - The Far Wildlands",
            "Team, It's Getting Serious!",
            "Lands Untamed",
            "Swamps Where Dreams Drown",
            "The Terror of the Swamps",
            "Cruest Beast, Devourer of Journeys",
            "Wasp Kingdom",
            "Elizant II's Promise",
            "Chapter 6 - Assault on Rubber Prison",
            "Forsaken Lands",
            "Termite Capitol",
            "DineMite Beats",
            "Mothiva's Grant Stand!",
            "Snug as a Bug in a Sub",
            "Flower Journey",
            "Mite Knight Game Start",
            "Mite Knight Theme",
            "Mite Knight Game Over",
            "Summer Holiday at the Metal Island™",
            "The Lab Over Snakemouth",
            "The Other One",
            "Rubber Prison",
            "Battle Against Ultimax, Who Has a Tank",
            "Chapter 7 - The Everlasting Sapling",
            "The Giants' Lair",
            "???",
            "The Usurper, Grasping for Power",
            "Transcending, Overpowering, Everlasting",
            "Team, We Dit It!",
            "A Virtuoso's Tribute",
            "Credits",
        ]
        .map(|s| s.into()),
    );
    albums.push(create_album(
        &mut rng,
        "/public/mock-covers/bug.jpg",
        "Bug Fables",
        "Tristan Alric",
        Some(titles),
        None,
    ));
    albums.push(create_album(
        &mut rng,
        "/public/mock-covers/ror.jpg",
        "Risk of Rain",
        "Chris Christodoulou",
        None,
        None,
    ));
    albums.push(create_album(
        &mut rng,
        "public/mock-covers/ror2.jpg",
        "Risk of Rain 2",
        "Chris Christodoulou",
        None,
        None,
    ));
    albums.push(create_album(
        &mut rng,
        "public/mock-covers/rim.png",
        "Rimworld",
        "Alistair Lindsay",
        None,
        None,
    ));
    albums.push(create_album(
        &mut rng,
        "public/mock-covers/mc.jpg",
        "Minecraft - Volume Alpha",
        "C418",
        None,
        None,
    ));
    albums.push(create_album(
        &mut rng,
        "public/mock-covers/mc2.jpg",
        "Minecraft - Volume Beta",
        "C418",
        None,
        None,
    ));
    albums.push(create_album(
        &mut rng,
        "public/mock-covers/sc.jpg",
        "SiIvaCraft - Volume Alpha Version & Beta Mix",
        "SiIvaGunner",
        None,
        None,
    ));
    albums
}

fn create_album(
    rng: &mut rand::rngs::ThreadRng,
    cover: &str,
    title: &str,
    artist: &str,
    titles: Option<Box<[String]>>,
    genre: Option<&str>,
) -> Album {
    use rand::Rng;
    let titles = titles.unwrap_or(Box::from_iter(
        (1..rng.gen_range(6..62)).map(|i| format!("Song {i}")),
    ));
    let songs = Box::from_iter(
        titles
            .iter()
            .enumerate()
            .map(|(i, t)| Song::new(t, &"Artist".into(), i, rng.gen_range(40..250_usize))),
    );
    Album::new(cover, title, artist, songs, genre.unwrap_or("Soundtrack"))
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
