use core::panic;
use anyhow::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug, Default)]
pub struct Flags {
    prepend_line_number: bool,
    names_only: bool,
    case_insensitive: bool,
    invert: bool,
    full_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f = Flags::default();
        for &flag in flags {
            match flag {
                "-n" => f.prepend_line_number = true,
                "-l" => f.names_only = true,
                "-i" => f.case_insensitive = true,
                "-v" => f.invert = true,
                "-x" => f.full_match = true,
                _ => panic!("Unknown flag {flag}")
            }
        }
        f
    }

}

fn match_line<'a>(line: &'a str, pattern: &str, case_insensitive: bool, full_match: bool) -> bool {
    let (line, pattern) = if case_insensitive {
        (line.to_lowercase(), pattern.to_lowercase())
    } else {
        (line.to_string(), pattern.to_string())
    };

    if full_match {
        line == pattern
    } else {
        line.contains(&pattern)
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = Vec::new();
    
    for &file_path in files {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if match_line(&line, pattern, flags.case_insensitive, flags.full_match) != flags.invert {
                if flags.names_only {
                    matches.push(file_path.to_string());
                    break;
                } else {
                    let mut result = String::new();
                    if files.len() > 1 {
                        result.push_str(file_path);
                        result.push(':');
                    }
                    if flags.prepend_line_number {
                        result.push_str(&format!("{}:", line_num + 1));
                    }
                    result.push_str(&line);
                    matches.push(result);
                }
            }
        }
    }
    
    Ok(matches)
}
