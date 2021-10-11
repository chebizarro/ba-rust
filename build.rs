use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// build script's entry point
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    let test_data_directories = read_dir("./tests/data/").unwrap();

    for directory in test_data_directories {
      for subdir in directory {
        write_header(&mut test_file, subdir.file_name().to_str().unwrap());
        for testfile in read_dir(subdir.path()).unwrap() {
          write_test(&mut test_file, &testfile.unwrap());
        }
      } 
    }
}

fn write_test(test_file: &mut File, directory: &DirEntry) {
  let directory = directory.path().canonicalize().unwrap();
  let path = directory.display();
  let test_name = format!(
        "test_{}",
        directory.file_name().unwrap().to_string_lossy()
      );

  write!(
      test_file,
      include_str!("./tests/test_template"),
      test_name = test_name,
      name = directory.file_name().unwrap().to_string_lossy(),
      path = path
  )
  .unwrap();
}

fn write_header(test_file: &mut File, module: &str) {
  
  write!(
    test_file,
    "use ba_rust::{}::*;\n",
    module
  )
  .unwrap();
}
