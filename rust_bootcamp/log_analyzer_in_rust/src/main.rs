use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use regex::Regex;

struct LogEntry {
    date: String,
    level: String,
    message: String,
}

fn parse_line(line: &str, re: &Regex) -> Option<LogEntry> {
    if let Some(caps) = re.captures(line) {
        Some(LogEntry {
            date: caps.get(1)?.as_str().to_string(),
            level: caps.get(2)?.as_str().to_string(),
            message: caps.get(3)?.as_str().to_string(),
        })
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <logfile> [output_summary.txt]", args[0]);
        std::process::exit(1);
    }

    let log_filename = &args[1];
    let summary_filename = if args.len() > 2 {
        &args[2]
    } else {
        "summary.txt"
    };

    let log_file = File::open(log_filename)
        .unwrap_or_else(|err| panic!("Error opening log file {}: {}", log_filename, err));
    let summary_file = File::create(summary_filename)?;
    let mut summary_writer = BufWriter::new(summary_file);
    let bad_lines_file = File::create("bad_lines.txt")?;
    let mut bad_lines_writer = BufWriter::new(bad_lines_file);

    let reader = BufReader::new(log_file);
    
    let re = Regex::new(r"^(\S+)\s+\S+\s+\[([^\]]+)\]\s+(.*)$")
        .expect("Failed to compile regex");

    let mut daily_counts: HashMap<String, (i32, i32, i32)> = HashMap::new();

    let mut error_freqs: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(entry) = parse_line(&line, &re) {
            let counts = daily_counts.entry(entry.date.clone()).or_insert((0, 0, 0));
            match entry.level.as_str() {
                "INFO" => counts.0 += 1,
                "WARNING" => counts.1 += 1,
                "ERROR" => counts.2 += 1,
                _ => {}
            }

            // Update error frequencies if the level is ERROR.
            if entry.level == "ERROR" {
                *error_freqs.entry(entry.message.clone()).or_insert(0) += 1;
            }
        } else {
            // Write lines that do not match the expected format.
            writeln!(bad_lines_writer, "{}", line)?;
        }
    }

    // Write the summary output.
    writeln!(summary_writer, "Log Summary By Date:")?;
    for (date, (info, warning, error)) in &daily_counts {
        writeln!(summary_writer, "{} - INFO: {}, WARNING: {}, ERROR: {}", date, info, warning, error)?;
    }
    writeln!(summary_writer, "\nMost Frequent Errors:")?;
    for (message, count) in &error_freqs {
        writeln!(summary_writer, "\"{}\" - {} occurrences", message, count)?;
    }

    Ok(())
}
