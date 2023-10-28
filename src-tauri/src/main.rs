// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
use database::*;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;

fn main() {
    let file = File::open("data.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut db: Vec<Data> = vec![];

    for result in rdr.records() {
        if let Ok(record) = result {
            let d = Data::new(
                record.get(0).unwrap(),
                record.get(2).unwrap(),
                record.get(1).unwrap(),
                record.get(3).unwrap(),
            );
            if !d.word.is_empty() && !d.meaning.is_empty() {
                db.push(d);
            }
        }
    }

    tauri::Builder::default()
        .manage(DatabaseState::new(db))
        .invoke_handler(tauri::generate_handler![word_prompt, meaning_prompt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WordPrompt {
    meaning: String,
    answer: String,
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
}

#[tauri::command]
fn word_prompt(db: tauri::State<'_, DatabaseState>) -> Result<WordPrompt, String> {
    let data = db.0.lock().unwrap();

    let mut rng = rand::thread_rng();
    let ans = rng.gen_range(0, 6);
    let mut points: Vec<Data> = vec![];
    for _ in 0..6 {
        let idx: usize = rng.gen_range(0, data.len());
        points.push(data.get(idx).unwrap().clone());
    }

    let prompt = WordPrompt {
        answer: "ABCDEF".chars().nth(ans).unwrap().to_string(),
        meaning: points.get(ans).unwrap().meaning.clone(),
        a: points.get(0).unwrap().word.clone(),
        b: points.get(1).unwrap().word.clone(),
        c: points.get(2).unwrap().word.clone(),
        d: points.get(3).unwrap().word.clone(),
        e: points.get(4).unwrap().word.clone(),
        f: points.get(5).unwrap().word.clone(),
    };

    Ok(prompt)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MeaningPrompt {
    word: String,
    answer: String,
    a: String,
    b: String,
    c: String,
    d: String,
}

#[tauri::command]
fn meaning_prompt(db: tauri::State<'_, DatabaseState>) -> Result<MeaningPrompt, String> {
    let data = db.0.lock().unwrap();

    let mut rng = rand::thread_rng();
    let ans = rng.gen_range(0, 4);
    let mut points: Vec<Data> = vec![];
    for _ in 0..4 {
        let idx: usize = rng.gen_range(0, data.len());
        points.push(data.get(idx).unwrap().clone());
    }

    let prompt = MeaningPrompt {
        answer: "ABCDEF".chars().nth(ans).unwrap().to_string(),
        word: points.get(ans).unwrap().word.clone(),
        a: points.get(0).unwrap().meaning.clone(),
        b: points.get(1).unwrap().meaning.clone(),
        c: points.get(2).unwrap().meaning.clone(),
        d: points.get(3).unwrap().meaning.clone(),
    };

    Ok(prompt)
}