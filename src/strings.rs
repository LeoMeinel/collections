/*
 * File: strings.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * Strings are stored as a collection of UTF-8 encoded bytes
 * UTF-8 is unicode, each character can be a different size of bytes
 */

use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn strings() {
    strings_examples();
    strings_push();
    strings_add_format();
    strings_to_char();
}

fn strings_to_char() {
    let _hello = String::from("Hello");
    // Cannot use integer indexing because of UTF-8 and char using Scalar values
    //let c = hello[0];
    // Bytes
    for b in "@~`".bytes() {
        println!("{}", b);
    }
    // Scalar
    for s in "@~`".chars() {
        println!("{}", s);
    }
    // Grapheme clusters
    for g in "@~`".graphemes(true) {
        println!("{}", g);
    }
}

fn strings_add_format() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Combine two String reference
    let _s3 = format!("{} {}", &s1, &s2);
    // Add String reference to String
    let _s4 = s1 + &s2;
}

fn strings_push() {
    let mut s = String::from("foo");
    s.push_str("bar");
    // Chars are also UTF-8 encoded
    s.push('@');
}

fn strings_examples() {
    let _s1 = String::new();
    let s2 = "initial contents";
    let _s3 = s2.to_string();
    let _s4 = String::from("initial contents");
}
