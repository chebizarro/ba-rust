use super::*;

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