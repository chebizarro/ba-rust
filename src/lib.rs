pub mod ba1;

use std::collections::HashMap;
//use common_macros::{hash_map};

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

fn check_if_pattern_is_clump(index_list: &Vec<usize>, max_diff: usize, t: usize) -> bool {
  if index_list.len() < t {
      return false
  }

  for i in 0..(index_list.len() - t + 1) {
      if index_list[i + t - 1] - index_list[i] <= max_diff {
          return true
      }
  }
  false
}

// Based on the solution posted here: 
// https://stepik.org/lesson/4/step/6?discussion=3034577&thread=solutions&unit=8233

pub fn find_clumps_fast(genome: &str, k: usize, l: usize, t: usize) -> Vec<&str> {
  let mut k_mer_map = HashMap::new();
  for i in 0 .. (genome.len() - k + 1) {
      (*k_mer_map.entry(&genome[i..i+k]).or_insert(Vec::<usize>::new())).push(i);
  }
  let max_diff = l - k;
  
  let result = k_mer_map.iter()
    .filter(|&(_, v)| check_if_pattern_is_clump(v, max_diff, t))
    .map(|(k, _)| *k)
    .collect();
  return result;
}

pub fn minimize_skew(genome: &str) -> Vec<i32> {
  let mut skew = vec![0; genome.len() + 1];

  for i in 0..genome.len() {
    let score = match &genome[i..i + 1] {
      "A" => skew[i],
      "C" => skew[i] - 1,
      "T" => skew[i],
      "G" => skew[i] + 1,
      _ => skew[i],
    };
    skew[i + 1] = score;
  }

  let min = skew.iter().min().unwrap();
  let mut min_vec = Vec::new();

  for i in 0..skew.len() {
    if skew[i] == *min {
      min_vec.push(i as i32);
    }
  }

  return min_vec;
}

pub fn hamming_distance(p: &str, q: &str) -> i32 {
  return p.chars()
          .zip(q.chars())
          .filter(|(pc, qc)| !pc.eq(qc))
          .count() as i32;
}

pub fn approximate_pattern_matching(text: &str, pattern: &str, d: i32) -> Vec<i32> {
  let mut positions = Vec::new();

  for i in 0..=(text.len()-pattern.len()) {
    let pat_p = &text[i..i+pattern.len()];
    if hamming_distance(pattern, pat_p) <= d {
      positions.push(i as i32);
    }
  }
  return positions;
}

pub fn approximate_pattern_count(text: &str, pattern: &str, d: i32) -> i32 {
  let mut count = 0;

  for i in 0..=(text.len()-pattern.len()) {
    let pat_p = &text[i..i+pattern.len()];
    if hamming_distance(pattern, pat_p) <= d {
      count += 1;
    }
  }
  return count;
}

pub fn frequent_words_with_mismatches(text: &str, k: usize, d: i32) -> Vec<String> {
  let mut patterns = Vec::new();
  let mut freq_map = HashMap::new();
  let n = text.len();

  for i in 0..=(n-k) {
    let pattern = String::from(&text[i..i+k]);

    let neighborhood = neighbors(&pattern, d);

    for neighbor in neighborhood {
      if !freq_map.contains_key(&neighbor) {
        freq_map.insert(neighbor, 1);
      } else {
        let c = freq_map.get(&neighbor).unwrap() + 1;
        freq_map.insert(neighbor, c);
      }
    }
  }

  let m = freq_map.values().max().unwrap();

  for (pattern,_) in &freq_map {
    if freq_map[pattern] == *m {
      let mut p = String::with_capacity(pattern.len());
      p.push_str(&pattern);
      patterns.push(p);
    }
  }

  return patterns;

}

/*
fn neighbors(pattern: String, d: i32) -> HashMap<String, i32> {

  return pattern.chars().enumerate().fold(
    hash_map!{pattern => 0},
    |r,(i, _)| {
      r.iter().flat_map(
      |(seq, d0)| {
        if *d0 == d {
          return r;
        } else {
          let current_base = &seq.chars().nth(i).unwrap();
          "ACGT".chars().map(|c| {
            if c == *current_base {
              r.insert(seq.to_string(), *d0);
              
            } else {
              let mut k = seq.to_string();
              k.remove(i);
              k.insert(i, c);
              r.insert(k, d0 + 1);
            }
          });
          return r;
        }});
      return r;
    });

}
*/

fn neighbors(pattern: &str, d: i32) -> Vec<String> {
  if d == 0 {
    return vec![pattern.to_string()];
  }
  let nucleotides = vec!["A".to_string(), "C".to_string(), "G".to_string(), "T".to_string()]; 
  if pattern.len() == 1 {
    return nucleotides;
  }
  let mut neighborhood = Vec::new();
  let suffix_neighbors = neighbors(&pattern[1..], d);

  for text in suffix_neighbors {
    if hamming_distance(&pattern[1..], text.as_str()) < d {
      for x in nucleotides.iter() {
        neighborhood.push([x, text.as_str()].concat());
      }
    } else {
      neighborhood.push([&pattern[0..1], text.as_str()].concat());
    }
  }
  return neighborhood;
}

pub fn frequent_words_with_mismatches_and_reverse_compliments(text: &str, k: usize, d: i32) -> Vec<String> {
  let mut patterns = Vec::new();
  let mut freq_map = HashMap::new();
  let n = text.len();

  for i in 0..=(n-k) {
    let pattern = String::from(&text[i..i+k]);

    let neighborhood = neighbors(&pattern, d);

    for neighbor in neighborhood {
      if !freq_map.contains_key(&neighbor) &&
        !freq_map.contains_key(reverse_compliment(neighbor.as_str()).as_str()){
        freq_map.insert(neighbor, 1);
      } else {
        let c = freq_map.get(&neighbor).unwrap() + 1;
        freq_map.insert(neighbor, c);
      }
    }
  }

  let m = freq_map.values().max().unwrap();

  for (pattern,_) in &freq_map {
    if freq_map[pattern] == *m {
      let mut p = String::with_capacity(pattern.len());
      p.push_str(&pattern);
      patterns.push(p);
    }
  }

  return patterns;

}