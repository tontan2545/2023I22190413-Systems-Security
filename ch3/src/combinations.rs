use itertools::Itertools;
use std::collections::HashMap;

pub fn generate_combinaton_catesian_product(password: &str) -> Vec<String> {
    let substitutions = HashMap::from([
        ('a', vec!['a', 'A', '4']),
        ('b', vec!['b', 'B', '8']),
        ('c', vec!['c', 'C']),
        ('d', vec!['d', 'D', '6']),
        ('e', vec!['e', 'E', '3']),
        ('f', vec!['f', 'F']),
        ('g', vec!['g', 'G', '9']),
        ('h', vec!['h', 'H']),
        ('i', vec!['i', 'I', '1']),
        ('j', vec!['j', 'J']),
        ('k', vec!['k', 'K']),
        ('l', vec!['l', 'L', '1']),
        ('m', vec!['m', 'M']),
        ('n', vec!['n', 'N']),
        ('o', vec!['o', 'O', '0']),
        ('p', vec!['p', 'P']),
        ('q', vec!['q', 'Q']),
        ('r', vec!['r', 'R']),
        ('s', vec!['s', 'S', '5']),
        ('t', vec!['t', 'T', '7']),
        ('u', vec!['u', 'U']),
        ('v', vec!['v', 'V']),
        ('w', vec!['w', 'W']),
        ('x', vec!['x', 'X']),
        ('y', vec!['y', 'Y']),
        ('z', vec!['z', 'Z', '2']),
    ]);

    let char_vectors: Vec<Vec<char>> = password
        .chars()
        .map(|c| {
            substitutions
                .get(&c.to_ascii_lowercase())
                .cloned()
                .unwrap_or_else(|| vec![c.to_ascii_lowercase()])
        })
        .collect();

    let all_combinations = char_vectors
        .into_iter()
        .multi_cartesian_product()
        .map(|chars| chars.into_iter().collect::<String>())
        .collect();

    all_combinations
}
