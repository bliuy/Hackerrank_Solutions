use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

mod bigger_is_greater;

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..T {
        let w = stdin_iterator.next().unwrap().unwrap();

        let result = bigger_is_greater::biggerIsGreater(&w);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
