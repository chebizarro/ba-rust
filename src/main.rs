//use itertools::Itertools;

fn main() {

}

fn pattern_count(text: &str, pattern: &str) -> i32 {
  let mut count = 0;

  for i in 0..=(text.len() - pattern.len()) {
    if text[i..i + pattern.len()].eq(&pattern[..]) {
      count = count + 1;
    }
  }

  return count;
}

fn frequent_words(text: &str, k: usize) -> Vec<&str> {
  let mut count = Vec::new();

  for i in 0..=(text.len() - k) {
    let pattern = &text[i..i + k];
    count.push(pattern_count(text, &pattern))
  }

  let max_count = count.iter().max().unwrap();
  let mut frequent_patterns = Vec::new();

  for i in 0..=(text.len() - k) {
    if count[i] == *max_count {
      frequent_patterns.push(&text[i..i + k]);
    }
  }
  frequent_patterns.sort();
  frequent_patterns.dedup();
  return frequent_patterns;
}

#[allow(dead_code)]
fn reverse_compliment(pattern: &str) -> String {
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

fn all_occurrences(pattern: &str, genome: &str) -> Vec<i32> {
  let mut occ = Vec::new();

  for i in 0..=(genome.len() - pattern.len()) {
    if genome[i..i + pattern.len()].eq(&pattern[..]) {
      occ.push(i as i32);
    }
  }

  return occ;
}

#[allow(dead_code)]
fn find_clumps(genome: &str, k: usize, l: usize, t: usize) -> Vec<&str> {
  let mut res = Vec::new();

  for i in 0..=genome.len() - l {
    let kmers = frequent_words(&genome[i..i + l], k);
    for kmer in kmers {
      if !res.contains(&kmer) {
        let c = all_occurrences(kmer, &genome[i..i + l]);
        if c.len() >= t {
          res.push(kmer);
        }
      }
    }
  }
  return res;
}

#[allow(dead_code)]
fn minimize_skew(genome: &str) -> Vec<i32> {
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

fn hamming_distance(p: &str, q: &str) -> i32 {
  return p
    .chars()
    .zip(q.chars())
    .filter(|(pc, qc)| !pc.eq(qc))
    .count() as i32;
}

pub fn approximate_pattern_count(text: &str, pattern: &str, d: i32) -> i32 {
  let mut count = 0;
  for i in 0..(text.len()-pattern.len()) {
    let pat_p = &text[i..i+pattern.len()];
    if hamming_distance(pattern, pat_p) <= d {
      count += 1;
    }
  }
  return count;
}

pub fn approximate_pattern_matching(_: &str, _: &str, _: i32) -> Vec<i32> {
  let positions = Vec::new();

  


  return positions;
}