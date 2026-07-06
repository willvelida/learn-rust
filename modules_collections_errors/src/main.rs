mod audio {
    pub mod playback {
        pub fn start_track() {
            println!("track started");
        }
    }
}

use crate::audio::playback;
use std::{collections::HashMap, hash::Hash};
use std::fs::{self, File};
use std::io;

fn main() {
    play_album();

    let mut scores: Vec<i32> = Vec::new();
    scores.push(10);
    scores.push(20);
    scores.push(30);

    for n in &scores {
        println!("{n}");
    }

    match scores.get(10) {
        Some(n) => println!("{n}"),
        None => println!("out of range"),
    }

    let mut s = String::from("hello");
    s.push_str(", world");
    let combined = format!("{s}");
    println!("{combined}");

    for ch in "hello".chars() {
        println!("{ch}");
    }

    let mut inventory = HashMap::new();
    inventory.insert(String::from("apples"), 12);

    let apples = inventory.get("apples").copied().unwrap_or(0);
    println!("apples: {apples}");

    inventory.entry(String::from("pears")).or_insert(20);
    println!("pears: {}", inventory["pears"]);

    match File::open("hello.txt") {
        Ok(_file) => println!("opened the file"),
        Err(_error) => println!("could not open the file"),
    }

    match read_username() {
        Ok(name) => println!("username: {name}"),
        Err(_error) => println!("could not read the username"),
    }
    
}

fn read_username() -> Result<String, io::Error> {
    let contents = fs::read_to_string("username.txt")?;
    Ok(contents.trim().to_string())
}

fn play_album() {
    playback::start_track();
}
