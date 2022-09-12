/*
 * File: vectors.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use self::enum_vectors::SpreadsheetCell;

mod enum_vectors;

pub(crate) fn vectors() {
    vector_initialize_and_push();
    vector_access_with_out_of_bounds();
    vector_access();
    vector_cannot_borrow_as_mutable();
    vector_loop();
    vector_loop_mutable();
    vector_loop_mutable_2();
    vector_enum();
}

fn vector_enum() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    match &row[2] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer!"),
    }
}

fn vector_loop_mutable_2() {
    let mut v = vec![1, -1, 6, -6];
    for i in &mut v {
        // Dereference
        *i += 50;
    }
    for i in &v {
        println!("{}", i)
    }
}

fn vector_loop_mutable() {
    let mut v = vec![1, -1, 6, -6];
    for i in &mut v {
        // Dereference
        *i += 50;
        println!("{}", i)
    }
}

fn vector_loop() {
    let v = vec![1, -1, 6, -6];
    for i in &v {
        println!("{}", i)
    }
}

#[allow(unused_mut)]
fn vector_cannot_borrow_as_mutable() {
    let mut v = vec![1, -1, 6, -6];
    let third = &v[2];
    // Cannot borrow as mutable because it is also borrowed as immutable
    //v.push(4);
    println!("The third element of v is: {}", third);
}

fn vector_access() {
    let v = vec![1, -1, 6, -6];
    match v.get(2) {
        Some(i) => println!("The third element of v is: {}", i),
        None => println!("There is no third element!"),
    }
}

fn vector_access_with_out_of_bounds() {
    let v = vec![1, -1, 6, -6];
    let third = &v[2];
    println!("The third element of v is: {}", third);
}

fn vector_initialize_and_push() {
    let _a = [1, -1];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(-1);
    // Initialize v2
    {
        let _v2 = vec![1, -1];
    }
    // End of scope, drop v2
}
