use super::*;
use common_macros::b_tree_map;

pub fn ba2a(input: &Vec<&str>) -> Vec<String> {
  
  let output = motif_enumeration(
    input[2..].iter().map(|s| *s).collect::<Vec<&str>>(),
    input[0].parse::<usize>().unwrap(),
    input[1].parse::<i32>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba2b(input: &Vec<&str>) -> Vec<String> {
  
  let output = median_string(
    input[1..].iter().map(|s| *s).collect::<Vec<&str>>(),
    input[0].parse::<usize>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba2c(input: &Vec<&str>) -> Vec<String> {
  
  let u = input[1].parse::<usize>().unwrap();

  let profile = b_tree_map!{
    "A".to_string() => input[2..=u+2].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
    "C".to_string() => input[u+2..u*2+2].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
    "G".to_string() => input[u*2+2..u*3+2].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
    "T".to_string() => input[u*3+2..].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
  };

  let output = most_probable(
    input[0],
    u,
    &profile
  );

  return vec![output];  
}

pub fn ba2d(input: &Vec<&str>) -> Vec<String> {
  
  let output = greedy_motif_search(
    input[2..].iter().map(|s| *s).collect::<Vec<&str>>(),
    input[0].parse::<usize>().unwrap(),
    input[1].parse::<usize>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn ba2e(input: &Vec<&str>) -> Vec<String> {
  
  let output = greedy_motif_search_with_pseudocounts(
    input[2..].iter().map(|s| *s).collect::<Vec<&str>>(),
    input[0].parse::<usize>().unwrap(),
    input[1].parse::<usize>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}

pub fn dbpas(input: &Vec<&str>) -> Vec<String> {

  let output = distance_between_pattern_and_strings(
    input[0],
    &input[1..].iter().map(|s| *s).collect::<Vec<&str>>()
  );

  let result = vec![output.to_string()];

  return result;  

}

pub fn ba2f(input: &Vec<&str>) -> Vec<String> {
  
  let i1 = input[2..].iter().map(|s| *s).collect::<Vec<&str>>();
  let i2 = input[0].parse::<usize>().unwrap();
  let i3 = input[1].parse::<usize>().unwrap();

  let mut output = Vec::new();

  for _ in 0..1000 {
    output.push(randomized_motif_search(
      &i1,
      i2,
      i3
    ));
  }

  let result = output.iter()
                    .min_by(|a, b| score(a).cmp(&score(b)))
                    .unwrap();

  return result.to_vec();  
}