// Rust implementation of http://rosalind.info/problems/ba1a/
use super::*;

pub fn ba1a(input: &Vec<&str>) -> Vec<String> {
  
  let mut result: Vec<String> = Vec::new();
  
  let output = pattern_count(input[0], input[1]).to_string(); 

  result.push(output);

  return result;
}

pub fn ba1b(input: &Vec<&str>) -> Vec<String> {
  
  let output = frequent_words(input[0], input[1].parse::<usize>().unwrap());

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba1c(input: &Vec<&str>) -> Vec<String> {
  
  let output = reverse_compliment(input[0]);

  let mut result = Vec::new();

  result.push(output);

  return result;  
}

pub fn ba1d(input: &Vec<&str>) -> Vec<String> {
  
  let output = all_occurrences(input[0], input[1]);

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn vibrio_cholerae(input: &Vec<&str>) -> Vec<String> {
  
  let output = all_occurrences(input[0], input[1]);

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba1e(input: &Vec<&str>) -> Vec<String> {

  let output = find_clumps(
    input[0],
    input[1].parse::<usize>().unwrap(),
    input[2].parse::<usize>().unwrap(),
    input[3].parse::<usize>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn e_coli(input: &Vec<&str>) -> Vec<String> {

  let output = find_clumps_fast(
    input[0],
    input[1].parse::<usize>().unwrap(),
    input[2].parse::<usize>().unwrap(),
    input[3].parse::<usize>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  println!("{}", output.len());

  return result;  
}

pub fn ba1f(input: &Vec<&str>) -> Vec<String> {

  let output = minimize_skew(input[0]);

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba1g(input: &Vec<&str>) -> Vec<String> {

  let output = hamming_distance(input[0], input[1]);

  let mut result = Vec::new();
  
  result.push(output.to_string());

  return result;
}

pub fn ba1h(input: &Vec<&str>) -> Vec<String> {

  let output = approximate_pattern_matching(
    input[1],
    input[0],
    input[2].parse::<i32>().unwrap()
  );

  println!("{}", output.len());

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba1h2(input: &Vec<&str>) -> Vec<String> {

  let output = approximate_pattern_count(
    input[1],
    input[0],
    input[2].parse::<i32>().unwrap()
  );

  let mut result = Vec::new();
  
  result.push(output.to_string());

  return result;
}
