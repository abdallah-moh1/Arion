// SPDX-License-Identifier: GPL-3.0-or-later
// Arion - Your space for thought.
// Copyright (C) 2026 Abdallah

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
}
