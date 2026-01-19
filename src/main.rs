use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    for line in input.lines() {
        let code_start = line
            .chars()
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(line.len());

        println!("{}", &line[code_start..]);
    }
}
