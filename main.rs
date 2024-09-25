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
    
    let mut lines_vec: Vec<String> = Vec::new(); // 벡터를 if let 바깥에서 선언

    if let Ok(lines) = read_lines(file_path) {        
        for line in lines {
            if let Ok(content) = line {
                lines_vec.push(content);
            }
        }
    } else {
        eprintln!("Could not read file: {}", file_path);
    }

    // 이제 여기서 lines_vec에 접근할 수 있습니다.
    for line in &lines_vec {
       if line.starts_with("byte") {

       }
    }
}

// 파일의 각 줄을 읽어오는 함수
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}