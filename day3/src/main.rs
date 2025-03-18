use lazy_regex::regex_captures_iter;

fn main() {
    let file = load_file("./src/input.txt");
	let mut lines = Vec::new();
    for line in regex_captures_iter!(r"(mul\(\d{0,3},\d{0,3}\))", &file) {
		lines.push(line.get(1).unwrap().as_str());
    }

	println!("{:#?}", lines);
}

pub fn load_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}
