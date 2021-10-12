pub mod ba1;

use std::collections::HashMap;


pub fn pattern_count(text: &str, pattern: &str) -> i32 {
  let mut count = 0;

  for i in 0..=(text.len() - pattern.len()) {
    if text[i..i + pattern.len()].eq(&pattern[..]) {
      count = count + 1;
    }
  }

  return count;
}

pub fn frequent_words(text: &str, k: usize) -> Vec<&str> {
  let mut count = Vec::new();

  for i in 0..=(text.len() - k) {
    let pattern = &text[i..i + k];
    count.push(pattern_count(text, &pattern))
  }

  let max_count = count.iter().max().unwrap();
  let mut frequent_patterns = Vec::new();

  for i in 0..(text.len() - k) {
    if count[i] == *max_count {
      frequent_patterns.push(&text[i..i + k]);
    }
  }
  frequent_patterns.sort();
  frequent_patterns.dedup();
  return frequent_patterns;
}

pub fn reverse_compliment(pattern: &str) -> String {
  return pattern
    .chars()
    .rev()
    .map(|n| match n {
      'A' => "T",
      'C' => "G",
      'T' => "A",
      'G' => "C",
      _ => "*",
    })
    .collect::<String>();
}

fn frequency_table(text: &str, k: usize) -> HashMap<&str, usize> {
  let mut freq_map = HashMap::new();
  let n = text.len();

  for i in 0..=(n-k) {
    let pattern = &text[i..i+k];
    if !freq_map.contains_key(&pattern) {
      freq_map.insert(pattern, 1);
    } else {
      freq_map.insert(pattern, freq_map.get(pattern).unwrap() + 1);
    }
  }
  return freq_map;
}

pub fn better_frequent_words(text: &str, k: usize) -> Vec<&str> {
  let mut frequent_patterns = Vec::new();
  let freq_map = frequency_table(text, k);
  let max = freq_map.values().max().unwrap();
  for (pattern, value) in &freq_map {
    if value == max {
      frequent_patterns.push(*pattern)
    }
  }

  return frequent_patterns;
}


pub fn all_occurrences(pattern: &str, genome: &str) -> Vec<i32> {
  let mut occ = Vec::new();

  for i in 0..=(genome.len() - pattern.len()) {
    if genome[i..i + pattern.len()].eq(&pattern[..]) {
      occ.push(i as i32);
    }
  }

  return occ;
}

pub fn find_clumps(text: &str, k: usize, l: usize, t: usize) -> Vec<&str> {
  let mut patterns = Vec::new();
  let n = text.len();

  for i in 0..=(n-l) {
    let window = &text[i..i+l];
    let freq_map = frequency_table(window, k);
    for (pattern, value) in &freq_map {
      if *value >= t {
        patterns.push(*pattern)
      }
    }
  }
  patterns.sort();
  patterns.dedup();
  return patterns;
}
