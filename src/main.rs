use std::fs;

fn main() {
  let current_dir = std::env::current_dir().unwrap();

  for entry in fs::read_dir(current_dir).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();

    if path.is_file() {
        print!("{}","\x1b[32m");        
        println!("🗋  {}", path.file_name().unwrap().to_str().unwrap());
    }
    else {
        print!("{}","\x1b[33m");
        println!("🗀  {}", path.file_name().unwrap().to_str().unwrap());
    }
  }
  print!("{}","\x1b[0m");
}


