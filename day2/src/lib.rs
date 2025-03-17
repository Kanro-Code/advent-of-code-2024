use std::path::PathBuf;

pub const INPUT: &str = "./src/input.txt";

pub struct Report {
    pub list: Vec<u8>,
}

impl Report {
    pub fn new(list: Vec<u8>) -> Self {
        Self { list }
    }

    pub fn is_sorted(&self) -> bool {
        self.list.is_sorted()
    }

    pub fn is_sorted_dec(&self) -> bool {
        self.list.is_sorted_by(|a, b| a >= b)
    }

    pub fn has_valid_diff(&self) -> bool {
        self.list.windows(2).all(|window| {
            if let &[a, b] = window {
                let diff = a.abs_diff(b);
                diff > 0 && diff <= 3
            } else {
                false
            }
        })
    }

    pub fn is_valid_sequence(&self) -> bool {
        (self.is_sorted() || self.is_sorted_dec()) && self.has_valid_diff()
    }
}

pub fn load_file(path: impl Into<PathBuf>) -> String {
    std::fs::read_to_string(path.into()).expect("Failed to read file")
}

pub fn split_file(file: String) -> Vec<Vec<u8>> {
    let mut holder = Vec::new();
    for line in file.lines() {
        let parts: Vec<u8> = line
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        holder.push(parts);
    }
    holder
}

pub fn load_and_create_reports(path: impl Into<PathBuf>) -> Vec<Report> {
    let file_content = load_file(path);
    let lines = split_file(file_content);
    lines.into_iter().map(Report::new).collect()
}

#[cfg(test)]
mod parts {
    use super::*;

    #[test]
    fn part1() {
        let reports = load_and_create_reports(INPUT);
        let total = reports.iter().filter(|e| e.is_valid_sequence()).count();

        println!("Solution for day 2, part 1 is: {total}.")
    }

    #[test]
    fn part2() {}
}
