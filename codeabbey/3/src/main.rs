use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();
    let rows = buffer.trim().parse::<i32>().unwrap();

    let mut results = Vec::new();

    for _ in 0..rows {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        let mut sum = 0;

        for n in buffer.as_str().split_whitespace() {
            sum += n.parse::<i32>().unwrap()
        }

        results.push(sum.to_string());
    }

    print!("{}", results.join(" "));
}

