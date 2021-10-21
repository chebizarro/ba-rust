use super::*;
use common_macros::hash_map;

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

  let profile = hash_map!{
    "A".to_string() => input[2..=u+2].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
    "C".to_string() => input[u+2..u*2+2].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
    "G".to_string() => input[u*2+2..u*3+2].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
    "T".to_string() => input[u*3+2..].iter().map(|f| f.parse::<f32>().unwrap()).collect(),
  };

  let output = most_probable(
    input[0],
    u,
    profile
  );

  return vec![output];  
}