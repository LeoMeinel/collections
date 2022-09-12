/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::hash_maps::hash_maps;
use crate::strings::strings;
use crate::vectors::vectors;

mod hash_maps;
mod strings;
mod vectors;

fn main() {
    vectors();
    strings();
    hash_maps();
}
