use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file-path>", args[0]);
        return;
    }

    let file_path = &args[1];
    
    if let Ok(lines) = read_lines(file_path) {
        let mut lines_vec: Vec<String> = Vec::new();
        
        for line in lines {
            if let Ok(content) = line {
                lines_vec.push(content);
            }
        }
       
    } else {
        eprintln!("Could not read file: {}", file_path);
    }
    //main
}

// 파일의 각 줄을 읽어오는 함수
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}