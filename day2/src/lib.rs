use std::path::PathBuf;

pub const INPUT: &str = "./src/input.txt";

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

pub fn is_sorted_desc_or_asc<T>(vec: &[T]) -> bool
where
    T: PartialOrd,
{
    vec.is_sorted() || vec.is_sorted_by(|a, b| a >= b)
}

pub fn has_valid_diff(vec: &[u8]) -> bool {
    vec.windows(2).all(|window| {
        if let &[a, b] = window {
            let diff = a.abs_diff(b);
            diff > 0 && diff <= 3
        } else {
            false
        }
    })
}

pub fn is_valid_sequence(vec: &[u8]) -> bool {
	is_sorted_desc_or_asc(vec) && has_valid_diff(vec)
}

#[cfg(test)]
mod parts {
    use super::*;

    #[test]
    fn part1() {
        let file = load_file(INPUT);
        let lines = split_file(file);

        let total = lines
            .iter()
            .filter(|e| is_valid_sequence(e))
            .count();

        println!("Solution for day 2, part 1 is: {total}.")
    }

    #[test]
    fn part2() {}
}
