/*
 * collections is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/collections/blob/main/LICENSE
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
