/*
 * File: hash_maps.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::collections::HashMap;

pub(crate) fn hash_maps() {
    hash_maps_initialize();
    hash_maps_loop();
    hash_maps_update();
    hash_maps_update_example();
}

fn hash_maps_update_example() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // ["Hello", "world,", "wonderful", "world!"]
    // Count occurrence of word in text
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // Dereference
        *count += 1;
    }
    println!("map consists of: {:#?}", map);
}

fn hash_maps_update() {
    let mut scores = HashMap::new();
    // Replace existing values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    // Do not replace existing values -> Skip 2nd line
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);
}

fn hash_maps_loop() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hash_maps_initialize() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    println!("scores consists of: {:#?}", scores);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score of team {} is: {}", team_name, score.unwrap());
}
