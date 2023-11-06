use serde::{Deserialize, Serialize};
// use tauri::async_runtime::Mutex;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub word: String,
    pub meaning: String,
    pub part: String,
    pub vec: Vec<f64>
}

impl Data {
    pub fn new(word: &str, meaning: &str, part: &str) -> Data {
        Data {
            word: word.to_string(),
            meaning: meaning.to_string(),
            part: part.to_string(),
            vec: vec![]
        }
    }
}

pub struct DatabaseState(pub Mutex<Vec<Data>>);

impl DatabaseState {
    pub fn new(db: Vec<Data>) -> DatabaseState {
        DatabaseState(Mutex::new(db))
    }
}
