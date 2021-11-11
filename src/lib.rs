#![feature(total_cmp)]
#![feature(map_first_last)]

pub mod ba1;
pub mod ba2;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use rand::{thread_rng, Rng};

use common_macros::b_tree_map;

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

  let mut freq_map = HashMap::new();
  let n = text.len();

  for i in 0..=(n-k) {
    let pattern = String::from(&text[i..i+k]);
    let neighborhood = neighbors(&pattern, d);

    for neighbor in neighborhood {
      if !freq_map.contains_key(&neighbor) {
        let rc = &reverse_compliment(&neighbor);
        let count = approximate_pattern_count(text, &neighbor, d) +
          approximate_pattern_count(text, rc, d);
        freq_map.insert(String::from(neighbor), count);
        freq_map.insert(String::from(rc), count);
      }
    }
  }

  let mut m = freq_map.values().max().unwrap();

  let m_max = freq_map.iter()
    .filter(|(_, count)| *count == m)
    .map(|(pattern, count)| (pattern.clone(), *count)) // + freq_map.get(&reverse_compliment(pattern.as_str())).unwrap()))
    .collect::<HashMap<String, i32>>();

  m = m_max.values().max().unwrap();

  let patterns = m_max.iter()
    .filter(|(_, count)| *count == m)
    .map(|(pattern, _)| pattern.clone())
    .collect::<Vec<String>>();

  return patterns;
}

pub fn motif_enumeration(dna: Vec<&str>, k: usize, d: i32) -> HashSet<String> {
  let mut patterns = HashSet::new(); 

  let first = dna[0];
  let n = first.len();

  for i in 0..=(n-k) {
    let pattern = String::from(&first[i..i+k]);
    let neighborhood = neighbors(&pattern, d);

    for neighbor in neighborhood {
      let count = dna[1..].iter()
        .filter(|text| approximate_pattern_count(text, &neighbor, d) > 0)
        .count();
      if count == (dna.len() -1) {
        patterns.insert(neighbor);
      }
    }
  }

  return patterns;
}

fn distance_between_pattern_and_strings(pattern: &str, dna: &Vec<&str>) -> i32 {
  let k = pattern.len();
  let mut distance = 0;

  for text in dna {
    let mut hamming_d = i32::MAX;
    let n = text.len();
    for i in 0..=(n-k) {
      let pattern_p = &text[i..i+k];
      if hamming_d > hamming_distance(pattern, pattern_p) {
        hamming_d = hamming_distance(pattern, pattern_p);
      }
    }
    distance += hamming_d;
  }
  return distance;
}

pub fn median_string(dna: Vec<&str>, k: usize) -> Vec<String> {
  let mut distance = i32::MAX;
  let nucleotides = vec!["A","C","G", "T"];
  let mut median = Vec::new();

  let patterns = nucleotides.iter()
    .map(|n| neighbors(&n.repeat(k), distance))
    .flat_map(|v| v.into_iter())
    .collect::<Vec<String>>();

  for pattern in patterns {
    if distance > distance_between_pattern_and_strings(&pattern, &dna) {
      distance = distance_between_pattern_and_strings(&pattern, &dna);
      median.push(pattern);
    }
  }

  return vec![median.last().unwrap().clone()];
}

pub fn most_probable(text: &str, k: usize, profile: &BTreeMap<String, Vec<f32>>) -> String {
  let n = text.len();
  let mut output = Vec::new();

  for i in 0..=(n-k) {
    let pattern = &text[i..i+k];

    let prob = pattern.chars()
      .enumerate()
      .fold(1.0, |acc, (x, c)| acc * profile[&c.to_string()][x]);

    output.push((pattern, prob));
  }

  return output.iter().rev()
    .max_by(|a, b| a.1.total_cmp(&b.1))
    .map(|(p, _v)| p)
    .unwrap()
    .to_string();
}

fn score(motifs: &Vec<String>) -> i32 {

  let profile = make_profile(motifs);
  let k = motifs[0].len();

  let consensus = (0..k).fold(
    String::with_capacity(k), |c, i| c + profile.keys()
    .max_by(|a, b| profile[*a][i].total_cmp(&profile[*b][i]))
    .unwrap()
  );

  return motifs.iter()
    .fold(0, |s, m| s + hamming_distance(&consensus, &m));
}

fn make_profile(motifs: &Vec<String>) -> BTreeMap<String, Vec<f32>> {

  let k = motifs[0].len();

  let mut score = b_tree_map!{
    'A' => vec![0.0; k],
    'C' => vec![0.0; k],
    'G' => vec![0.0; k],
    'T' => vec![0.0; k],
  };
  
  for s in motifs {
    for (x, c) in s.chars().enumerate() {
      (*score.entry(c).or_insert(vec![0.0; k]))[x] += 1.0;
    }
  }

  let l = motifs.len() as f32;  

  return score.iter()
    .map(|(m, s)| (m.to_string(),
      s.iter()
      .map(|v| v / l)
      .collect::<Vec<f32>>()))
    .collect::<BTreeMap<String, Vec<f32>>>();
}

pub fn greedy_motif_search(dna: Vec<&str>, k: usize, t: usize) -> Vec<String> {

  let mut best_motifs = dna.iter()
    .map(|s| s[0..k].to_string())
    .collect::<Vec<String>>();

  for motif in (0..=(dna[0].len()-k)).map(|i| &dna[0][i..i+k]) {
    let mut motifs = vec![motif.to_string()];
    for idx in 1..t {
      let profile = make_profile(&motifs);
      let mp = most_probable(dna[idx], k, &profile);
      motifs.push(mp);
    }

    if score(&motifs) < score(&best_motifs) {
      best_motifs = motifs;
    }
  }

  return best_motifs;
}

fn make_pseudo_profile(motifs: &Vec<String>) -> BTreeMap<String, Vec<f32>> {

  let k = motifs[0].len();

  let mut score = b_tree_map!{
    'A' => vec![1.0; k],
    'C' => vec![1.0; k],
    'G' => vec![1.0; k],
    'T' => vec![1.0; k],
  };
  
  for s in motifs {
    for (x, c) in s.chars().enumerate() {
      (*score.entry(c).or_insert(vec![1.0; k]))[x] += 1.0;
    }
  }

  let l = motifs.len() as f32;  

  return score.iter()
    .map(|(m, s)| (m.to_string(),
      s.iter()
      .map(|v| v / l)
      .collect::<Vec<f32>>()))
    .collect::<BTreeMap<String, Vec<f32>>>();
}

pub fn greedy_motif_search_with_pseudocounts(dna: Vec<&str>, k: usize, t: usize) -> Vec<String> {

  let mut best_motifs = dna.iter()
    .map(|s| s[0..k].to_string())
    .collect::<Vec<String>>();

  for motif in (0..=(dna[0].len()-k)).map(|i| &dna[0][i..i+k]) {
    let mut motifs = vec![motif.to_string()];
    for idx in 1..t {
      let profile = make_pseudo_profile(&motifs);
      let mp = most_probable(dna[idx], k, &profile);
      motifs.push(mp);
    }

    if score(&motifs) < score(&best_motifs) {
      best_motifs = motifs;
    }
  }

  return best_motifs;
}


fn make_motifs(dna: &Vec<&str>, profile: BTreeMap<String, Vec<f32>>) ->Vec<String> {

  let l = dna[0].len();
  let k = profile.first_key_value().unwrap().1.len();

    let mut motifs = Vec::new();
    
    for strand in dna {
      let mp = most_probable(strand, k, &profile);
      motifs.push(mp);
    }
    
    return motifs;

}

pub fn randomized_motif_search(dna: Vec<&str>, k: usize, t: usize) -> Vec<String> {

  let mut rng = thread_rng();
  let l = dna[0].len();

  let mut motifs = dna.iter()
    .map(|s| s[rng.gen_range(0..l-k)..k].to_string())
    .collect::<Vec<String>>();

  let mut best_motifs = motifs.iter().map(|s| s.to_owned()).collect();

  loop {
    let profile = make_pseudo_profile(&motifs);
    motifs = make_motifs(&dna, profile);

    if score(&motifs) < score(&best_motifs) {
      best_motifs = motifs.iter().map(|s| s.to_owned()).collect();;
    } else {
      return best_motifs;
    }
  }
}