use std::cmp::Ordering;
use std::path::PathBuf;

fn main() {
	let file = load_file("./src/input.txt".to_string());

	let (mut l1, mut l2) = split_file(&file);
	l1.sort();
	l2.sort();

	let mut total = 0;

	l1.iter().zip(l2.iter()).for_each(|(a,b)| {
		match a.cmp(b) {
			Ordering::Greater => { total += a - b},
			Ordering::Less => { total += b - a},
			_ => {},
		}
 	});

	println!("{total}")
}

fn load_file(path: impl Into<PathBuf>) -> String {
	std::fs::read_to_string(path.into()).expect("Failed to read file")
}

fn split_file(file: &str) -> (Vec<usize>, Vec<usize>) {
	let mut list1 = Vec::new();
	let mut list2 = Vec::new();

	for line in file.lines() {
		let mut parts = line.split_whitespace().map(|s| s.parse::<usize>());

		if let (Some(Ok(a)), Some(Ok(b))) = (parts.next(), parts.next()) {
			list1.push(a);
			list2.push(b);
		}
	}

	(list1, list2)
}
