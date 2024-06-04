use super::{Album, Song};

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
