pub mod ba1;

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

pub fn all_occurrences(pattern: &str, genome: &str) -> Vec<i32> {
  let mut occ = Vec::new();

  for i in 0..=(genome.len() - pattern.len()) {
    if genome[i..i + pattern.len()].eq(&pattern[..]) {
      occ.push(i as i32);
    }
  }

  return occ;
}