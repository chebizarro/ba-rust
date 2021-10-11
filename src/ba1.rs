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
