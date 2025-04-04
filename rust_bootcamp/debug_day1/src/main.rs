use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
    }

    Ok(lines)
}

fn count_words(lines: &[String]) -> usize {
    let mut count = 0;
    for line in lines {
        // Intentional logic bug: splits on every character
        let words = line.split_whitespace();
        count += words.count();

    }
    count
}

fn average_line_length(lines: &[String]) -> f64 {
    let total_chars: usize = lines.iter().map(|line| line.len()).sum();
    match lines.len() {
        0 => panic!("NO LINES IN THE FILE"),
        _ => {
            let avg = total_chars / lines.len(); // 💥 will panic if file is empty
            avg as f64
        }
    }

}

fn report_summary(file_lines: Vec<String>) {
    // ❌ Intentional compile-time error: undeclared variable `lines`
    println!("Number of lines: {}", file_lines.len());
    println!("Total word count: {}", count_words(&file_lines));
    println!("Average line length: {:.2}", average_line_length(&file_lines));
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let lines = read_lines(filename)?;
    report_summary(lines);

    Ok(())
}
