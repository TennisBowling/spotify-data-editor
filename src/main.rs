use std::{
    fs::File,
    io::{Read, Write},
};

use serde_json::{self, Value};
fn main() {
    let matches = clap::App::new("spotify-data-editor")
        .version("0.0.1")
        .author("TennisBowling <tennisbowling@tennisbowling.com>")
        .setting(clap::AppSettings::ColoredHelp)
        .about("A simple program to edit your Spotify data")
        .long_version(
            "spotify-data-editor version 0.0.1 by TennisBowling <tennisbowling@tennisbowling.com>",
        )
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("remove-artist")
                .short("ra")
                .long("remove-artist-songs")
                .help("removes all songs by the specified artist")
                .takes_value(true)
                .required(true),
        );
    let matches = matches.get_matches();
    let input = matches.value_of("input").unwrap();
    let default_output = format!("{}-edited", input);
    let output = matches
        .value_of("output")
        .unwrap_or(default_output.as_str());
    let remove_artist = matches.value_of("remove-artist").unwrap_or("");
    println!("Input: {}", input);
    println!("Output: {}", output);
    println!("Remove Artist: {}", remove_artist);

    let mut file = File::open(input).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    /*
        from the spotify docs the format is a list of these:
        Here is an example of one song (end_song) streaming data:
    {
     "ts": "YYY-MM-DD 13:30:30",
     "username": "_________",
     "platform": "_________",
     "ms_played": _________,
     "conn_country": "_________",
     "ip_addr_decrypted": "___.___.___.___",
     "user_agent_decrypted": "_________",
     "master_metadata_track_name": "_________,
     “master_metadata_album_artist_name:_________”,
     “master_metadata_album_album_name:_________",
     “spotify_track_uri:_________”,
     "episode_name": _________,
     "episode_show_name": _________,
     “spotify_episode_uri:_________”,
     "reason_start": "_________",
     "reason_end": "_________",
     "shuffle": null/true/false,
     "skipped": null/true/false,
     "offline": null/true/false,
     "offline_timestamp": _________,
     "incognito_mode": null/true/false,
    } */
    let mut data: Value = serde_json::from_str(&contents).expect("Unable to parse JSON");

    println!("Beginning to remove songs by {}", remove_artist);

    let mut removed: i32 = 0;
    for song in data.as_array_mut().unwrap() {
        let artist = song["master_metadata_album_artist_name"].as_str();

        if artist == Some(remove_artist) {
            println!("Removing song: {}", song["master_metadata_track_name"]);
            // simply delete the song from the array
            song.as_object_mut().unwrap().clear();
            removed += 1;
        } else {
            continue;
        }
    }
    // remove all empty songs
    data.as_array_mut().unwrap().retain(|song| !song.is_null());

    println!("Removed {} songs by {}", removed, remove_artist);

    let mut file = File::create(output).expect("Unable to create output file");
    println!("Writing to file {}", output);
    file.write(data.to_string().as_bytes())
        .expect("Unable to write to file");

    println!("Done!");
}
