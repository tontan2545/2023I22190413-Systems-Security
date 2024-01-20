mod combinations;
mod core;
mod hash;

use crate::core::find_hash;
use hash::hash;
use rayon::prelude::*;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    sync::atomic::Ordering,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

// Hash function timing benchmark
// fn main() {
//     let tick = std::time::Instant::now();
//     hash("password");

//     println!("Hashing time: {:?}", tick.elapsed());
// }

fn main() -> std::io::Result<()> {
    static FILENAME: &str = "./data/common-passwords.txt";
    static OUT_FILE: &str = "./data/password.txt";
    static DESIRED_HASH: &str = "d54cc1fe76f5186380a0939d2fc1723c44e8a5f7";
    static CHUNK_SIZE: usize = 100;

    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);

    let results: Arc<Mutex<Vec<_>>> = Arc::new(Mutex::new(Vec::new()));
    let is_found = Arc::new(AtomicBool::new(false));

    let tick = std::time::Instant::now();

    reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .chunks(CHUNK_SIZE)
        .par_bridge()
        .for_each(|chunk| {
            // Finding hash
            if is_found.load(Ordering::Relaxed) {
                return;
            }
            for password in chunk {
                if let Some(result) = find_hash(&password, &is_found, DESIRED_HASH) {
                    let mut results_lock = results.lock().unwrap();
                    results_lock.push(result);
                    is_found.store(true, Ordering::Relaxed);
                    break;
                }
            }

            // Creating rainbow tables

            // chunk
            //     .iter()
            //     .map(|password| generate_hash_pw(password))
            //     .for_each(|pair| {
            //         let mut results_lock = results.lock().unwrap();
            //         results_lock.extend(pair);
            //     });
        });

    println!("Password computed time: {:?}", tick.elapsed());

    let results_lock = results.lock().unwrap();
    let mut file = OpenOptions::new().write(true).create(true).open(OUT_FILE)?;

    // Find hash
    for pw in &*results_lock {
        writeln!(file, "{}", pw)?;
    }

    // Rainbow table
    // for (hashed_pw, pw) in &*results_lock {
    //     writeln!(file, "{}: {}", hashed_pw, pw)?;
    // }

    println!("File writing time: {:?}", tick.elapsed());

    Ok(())
}
