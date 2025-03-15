use std::path::PathBuf;

pub fn get_lists() -> (Vec<u64>, Vec<u64>) {
    let path = PathBuf::from("./src/input.txt");
    let string = std::fs::read_to_string(path).expect("Failed to read file");
    split_file(&string)
}

pub fn split_file(file: &str) -> (Vec<u64>, Vec<u64>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in file.lines() {
        let mut parts = line.split_whitespace().map(|s| s.parse::<u64>());

        if let (Some(Ok(a)), Some(Ok(b))) = (parts.next(), parts.next()) {
            list1.push(a);
            list2.push(b);
        } else {
			panic!("Failed to parse line: {line}")
		}
    }

	list1.sort();
	list2.sort();

    (list1, list2)
}

pub fn calculate_total_difference(list1: &[u64], list2: &[u64]) -> u64 {
	list1.iter()
		.zip(list2.iter())
		.map(|(a, b)| a.abs_diff(*b))
		.sum()
}

#[cfg(test)]
mod parts {
    use super::*;
	use std::collections::HashMap;

    #[test]
    pub fn part1() {
        let (list1, list2) = get_lists();
        let total = calculate_total_difference(&list1, &list2);
        println!("The answer to part 1 is: {total}")
    }

    #[test]
    pub fn part2() {
        let (list1, list2) = get_lists();

        let mut hash: HashMap<u64, u64> = HashMap::new();

        for key in list2 {
            *hash.entry(key).or_insert(0) += 1;
        }

        let mut count = 0;

        for key in list1 {
            if hash.contains_key(&key) {
                count += key * hash.get(&key).unwrap();
            }
        }

        println!("The answer to part 2 is: {count}")
    }
}
