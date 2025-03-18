use lazy_regex::regex_captures_iter;

fn main() {
    let file = load_file("./src/input.txt");
	let mut lines: Vec<u64> = Vec::new();
    for line in regex_captures_iter!(r"mul\((\d{0,3}),(\d{0,3})\)", &file) {
		let (a, b) = (line.get(1).unwrap().as_str().parse::<u64>().unwrap(), line.get(2).unwrap().as_str().parse::<u64>().unwrap());
		lines.push(a * b);
    }

	let total: u64 = lines.iter().sum();
	println!("Day 3 Part 1: {}", total);
}

pub fn load_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}
