use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::combinations::generate_combinaton_catesian_product;
use crate::hash;

pub fn find_hash(password: &String, is_found: &AtomicBool, hash: &str) -> Option<String> {
    let combinations = generate_combinaton_catesian_product(password);

    for combination in combinations {
        if is_found.load(Ordering::Relaxed) {
            return None; // Early exit if result is found
        }
        if hash::hash(&combination) == hash {
            return Some(combination);
        }
    }

    None
}

pub fn generate_hash_pw(password: &str) -> Vec<(String, String)> {
    let combinations = generate_combinaton_catesian_product(password);

    let mut hash_pw_pairs: Vec<(String, String)> = Vec::new();

    for combination in combinations {
        hash_pw_pairs.push((hash::hash(&combination), combination));
    }

    hash_pw_pairs
}
