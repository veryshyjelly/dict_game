// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod database;
use commands::*;
use database::*;

use std::{
    collections::HashMap,
    fs::{self, File},
};

fn main() {
    tauri::Builder::default()
        .manage(DatabaseState::new(get_db("data.csv", "model.txt")))
        .invoke_handler(tauri::generate_handler![word_prompt, meaning_prompt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_db(file_name: &str, model_path: &str) -> Vec<Data> {
    let mut word2vec: HashMap<String, Vec<f64>> = HashMap::new();
    fs::read_to_string(model_path)
        .unwrap()
        .replace("\r", "")
        .split("\n")
        .skip(1)
        .for_each(|word_vec| {
            let mut word_iter = word_vec.split(" ").into_iter();
            let word = word_iter.next().unwrap();
            word2vec.insert(
                word.to_string(),
                word_iter
                    .map(|v| v.parse().expect("error while parsing"))
                    .collect::<Vec<f64>>(),
            );
        });

    let file = File::open(file_name).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut db: Vec<Data> = vec![];

    rdr.records().for_each(|result| {
        if let Ok(record) = result {
            let mut d = Data::new(
                record.get(0).unwrap(),
                record.get(2).unwrap(),
                record.get(1).unwrap(),
            );
            if !d.word.is_empty() && !d.meaning.is_empty() {
                if let Some(vector) = word2vec.get(&d.word) {
                    d.vec = vector.clone();
                    db.push(d);
                }
            }
        }
    });

    db
}
