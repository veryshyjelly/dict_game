use std::{cmp::Reverse, collections::BinaryHeap};

use super::database::*;
use rand::{self, seq::SliceRandom, thread_rng, Rng};
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

struct Point {
    index: usize,
    similarity: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.similarity.total_cmp(&other.similarity))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.similarity.total_cmp(&other.similarity)
    }
}

#[tauri::command]
pub fn word_prompt(db: tauri::State<'_, DatabaseState>) -> Result<WordPrompt, String> {
    println!("called this function");
    let data = db.0.lock().unwrap();

    let mut rng = rand::thread_rng();
    let prompt_idx = rng.gen_range(0, data.len());
    let ans_word = data.get(prompt_idx).unwrap();

    let mut heap: BinaryHeap<Reverse<Point>> = BinaryHeap::new();

    for i in 0..data.len() {
        let det = data.get(i).unwrap();
        if det.word.cmp(&ans_word.word).is_eq() {
            continue;
        }
        let cosine_similarity = cosine(&ans_word.vec, &det.vec);
        heap.push(Reverse(Point {
            index: i,
            similarity: cosine_similarity,
        }));
        if heap.len() > 15 {
            heap.pop();
        }
    }

    let ans = rng.gen_range(0, 6);
    let mut points: Vec<String> = vec!["".to_string(); 6];
    let mut plen = 0;
    let mut heap_vec = heap.into_iter().map(|v| v.0).collect::<Vec<_>>();
    heap_vec.sort();

    while plen < 6 {
        let det = data.get(heap_vec.pop().unwrap().index).unwrap();
        if points.contains(&det.word) {
            continue;
        }
        points[plen] = det.word.clone();
        plen += 1;
    }

    points.shuffle(&mut thread_rng());
    points[ans] = ans_word.word.clone();

    Ok(WordPrompt {
        answer: "ABCDEF".chars().nth(ans).unwrap().to_string(),
        meaning: ans_word.meaning.clone(),
        pos: ans_word.part.clone(),
        a: points[0].clone(),
        b: points[1].clone(),
        c: points[2].clone(),
        d: points[3].clone(),
        e: points[4].clone(),
        f: points[5].clone(),
    })
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
pub fn meaning_prompt(_db: tauri::State<'_, DatabaseState>) -> Result<MeaningPrompt, String> {
    // let data = db.0.lock().unwrap();

    // let mut rng = rand::thread_rng();
    // let prompt_idx = rng.gen_range(0, data.len());
    // let ans_word = data.get(prompt_idx).unwrap();

    // let mut alike: Vec<(usize, f64)> = vec![(0, f64::MAX); 10];
    // for i in 0..data.len() {
    //     if i == prompt_idx {
    //         continue;
    //     }
    //     let cosine_similarity = cosine(&ans_word.vec, &data.get(i).unwrap().vec);
    //     if let Some(idx) = smallest_bigger(&alike, cosine_similarity) {
    //         alike[idx] = (i, cosine_similarity);
    //     }
    // }

    // let ans = rng.gen_range(0, 4);
    // let mut points: Vec<usize> = vec![];
    // while points.len() < 4 {
    //     let idx: usize = alike.get(rng.gen_range(0, 10)).unwrap().0;
    //     if points.contains(&idx) {
    //         continue;
    //     }
    //     points.push(idx);
    // }

    // points.shuffle(&mut thread_rng());
    // points[ans] = prompt_idx;

    // Ok(MeaningPrompt {
    //     answer: "ABCDEF".chars().nth(ans).unwrap().to_string(),
    //     word: ans_word.word.clone(),
    //     a: data.get(points[0]).unwrap().meaning.clone(),
    //     b: data.get(points[1]).unwrap().meaning.clone(),
    //     c: data.get(points[2]).unwrap().meaning.clone(),
    //     d: data.get(points[3]).unwrap().meaning.clone(),
    // })
    Err("Not implemented yet".to_string())
}
