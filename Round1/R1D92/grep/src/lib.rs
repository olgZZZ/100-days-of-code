use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Error;

#[derive(Debug)]

pub struct Flags {
    line_numbers: bool,
    case_insensitive: bool,
    only_files: bool,
    whole_lines: bool,
    inverted: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let line_numbers = flags.contains(&"-n");
        let case_insensitive = flags.contains(&"-i");
        let only_files = flags.contains(&"-l");
        let whole_lines = flags.contains(&"-x");
        let inverted = flags.contains(&"-v");

        Flags {
            line_numbers,
            case_insensitive,
            only_files,
            whole_lines,
            inverted,
        }
    }
}

fn format_line(
    line: &str,
    number: usize,
    file_name: &str,
    flags: &Flags,
    multiple_files: bool,
) -> String {
    let mut output_line = line.to_string();

    if flags.line_numbers {
        output_line = format!("{}:{}", number + 1, line);
    }

    if multiple_files {
        output_line = format!("{}:{}", file_name, output_line);
    }

    output_line
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result: Vec<String> = vec![];
    let mut inverted_result: Vec<String> = vec![];
    let mut pattern = pattern.to_string();
    let multiple_files = files.len() > 1;

    for file_name in files {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        for (l_number, line) in reader.lines().enumerate() {
            let mut file_line = line.unwrap();
            let mut line = file_line.clone();

            if flags.case_insensitive {
                file_line = file_line.to_lowercase();

                pattern = pattern.to_lowercase();
            }

            line = format_line(line.as_str(), l_number, file_name, flags, multiple_files);

            if (file_line.contains(&pattern) && !flags.whole_lines)
                || (file_line == pattern && flags.whole_lines)
            {
                if flags.only_files {
                    if !result.contains(&file_name.to_string()) {
                        result.push(file_name.to_string());
                    }
                } else {
                    result.push(line);
                }
            } else {
                inverted_result.push(line);
            }
        }
    }

    if !flags.inverted {
        Ok(result)
    } else {
        Ok(inverted_result)
    }
}
