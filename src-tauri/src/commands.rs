use super::database::*;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordPrompt {
    meaning: String,
    answer: String,
    pos: String,
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
}

fn cosine(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    assert!(a.len() == b.len());

    let mut dot: f64 = 0.0;
    let mut norm_a: f64 = 0.0;
    let mut norm_b: f64 = 0.0;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    norm_a = norm_a.sqrt();
    norm_b = norm_b.sqrt();

    dot / (norm_a * norm_b)
}

fn smallest_bigger(alike: &Vec<(usize, f64)>, simi: f64) -> Option<usize> {
    let size = alike.len();
    for i in 0..size {
        if alike[i].0 == 0 {
            return Some(i);
        }
    }

    let mut max = 0.0;
    let mut max_index = alike.len();
    for i in 0..size {
        if (alike[i].1 < simi) && (alike[i].1 > max) {
            max = alike[i].1;
            max_index = i;
        }
    }

    if max_index < size {
        Some(max_index) 
    } else {
        None
    }

}

#[tauri::command]
pub fn word_prompt(db: tauri::State<'_, DatabaseState>) -> Result<WordPrompt, String> {
    let data = db.0.lock().unwrap();

    let mut rng = rand::thread_rng();
    let prompt_idx = rng.gen_range(0, data.len());
    let ans_word = data.get(prompt_idx).unwrap();

    let mut alike: Vec<(usize, f64)> = vec![(0, f64::MAX); 10];
    for i in 0..data.len() {
        let det = data.get(i).unwrap();
        if det.word.cmp(&ans_word.word).is_eq() {
            continue;
        }
        let cosine_similarity = cosine(&ans_word.vec, &det.vec);
        // println!("similarity: {}", cosine_similarity);
        if let Some(idx) = smallest_bigger(&alike, cosine_similarity) {
            alike[idx] = (i, cosine_similarity); 
        }
    }

    let ans = rng.gen_range(0, 6);
    let mut points: Vec<usize> = vec![];
    while points.len() < 6 {
        let idx: usize = alike.get(rng.gen_range(0, 10)).unwrap().0;
        if points.contains(&idx) || idx == prompt_idx {
            continue;
        }
        points.push(idx);
    }

    points[ans] = prompt_idx;

    let prompt = WordPrompt {
        answer: "ABCDEF".chars().nth(ans).unwrap().to_string(),
        meaning: ans_word.meaning.clone(),
        pos: ans_word.part.clone(),
        a: data.get(points[0]).unwrap().word.clone(),
        b: data.get(points[1]).unwrap().word.clone(),
        c: data.get(points[2]).unwrap().word.clone(),
        d: data.get(points[3]).unwrap().word.clone(),
        e: data.get(points[4]).unwrap().word.clone(),
        f: data.get(points[5]).unwrap().word.clone(),
    };

    Ok(prompt)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeaningPrompt {
    word: String,
    answer: String,
    a: String,
    b: String,
    c: String,
    d: String,
}

#[tauri::command]
pub fn meaning_prompt(db: tauri::State<'_, DatabaseState>) -> Result<MeaningPrompt, String> {
    let data = db.0.lock().unwrap();

    let mut rng = rand::thread_rng();
    let prompt_idx = rng.gen_range(0, data.len());
    let ans_word = data.get(prompt_idx).unwrap();

    let mut alike: Vec<(usize, f64)> = vec![(0, f64::MAX); 10];
    for i in 0..data.len() {
        if i == prompt_idx {
            continue;
        }
        let cosine_similarity = cosine(&ans_word.vec, &data.get(i).unwrap().vec);
        // println!("similarity: {}", cosine_similarity);
        if let Some(idx) = smallest_bigger(&alike, cosine_similarity) {
            alike[idx] = (i, cosine_similarity); 
        }
    }

    let ans = rng.gen_range(0, 6);
    let mut points: Vec<usize> = vec![];
    while points.len() < 6 {
        let idx: usize = alike.get(rng.gen_range(0, 10)).unwrap().0;
        if points.contains(&idx) {
            continue;
        }
        points.push(idx);
    }

    points[ans] = prompt_idx;

    let prompt = MeaningPrompt {
        answer: "ABCDEF".chars().nth(ans).unwrap().to_string(),
        word: ans_word.word.clone(),
        a: data.get(points[0]).unwrap().meaning.clone(),
        b: data.get(points[1]).unwrap().meaning.clone(),
        c: data.get(points[2]).unwrap().meaning.clone(),
        d: data.get(points[3]).unwrap().meaning.clone(),
    };

    Ok(prompt)
}
