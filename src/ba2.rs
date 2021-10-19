pub fn ba2a(input: &Vec<&str>) -> Vec<String> {
  
  let output = neighbors(
    input[0],
    input[1].parse::<i32>().unwrap()
  );

  let result = output.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

  return result;  
}