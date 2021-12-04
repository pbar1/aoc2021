use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_ints<P>(filename: P) -> Vec<i32>  where P: AsRef<Path> {
    let mut v: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(text) = line {
                let my_int: i32 = text.parse().unwrap();
                v.push(my_int);
            }
        }
    }
    v
}
