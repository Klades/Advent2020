use std::collections::HashSet;

pub fn part1(data: &Vec<String>) -> usize {
	return process_data(data);
}

pub fn part2(data: &Vec<String>) -> usize {
	return process_all_data(data);
}

fn parse_group_questions(data: &Vec<&str>) -> HashSet<char> {
	let mut set = HashSet::<char>::new();
	for line in data.iter() {
		for c in line.chars() {
			set.insert(c);
		}
	}

	return set;
}

fn parse_all_group_questions(data: &Vec<&str>) -> HashSet<char> {
	let mut set = HashSet::<char>::new();

	for c in data.first().unwrap().chars() {
		set.insert(c);
	}

	for line in data[1..].iter() {
		let mut small_set = HashSet::<char>::new();
		for c in line.chars() {
			small_set.insert(c);
		}
		
		let mut noballs = HashSet::<char>::new();
		for c in small_set.intersection(&set) {
			noballs.insert(*c);
		}
		set = noballs;
	}

	return set;
}

struct DataProcessor<'a> {
	lines: Vec<&'a str>,
}

impl<'a> DataProcessor<'a> {
	fn add(& mut self, line: &'a str) {
		self.lines.push(line);
	}

	fn process(& mut self) -> HashSet<char> {
		let set = parse_group_questions(&self.lines);
		self.lines.clear();
		return set;
	}

	fn process_all(& mut self) -> HashSet<char> {
		let set = parse_all_group_questions(&self.lines);
		self.lines.clear();
		return set;
	}
}

fn process_data(data: &Vec<String>) -> usize {
	let mut total = 0;
	let mut processor = DataProcessor { lines: Vec::new() };

	for line in data {
		match line.as_str() {
			"" => total += processor.process().len(),
			x => processor.add(x)
		}	
	}
	total += processor.process().len();

	return total;
}

fn process_all_data(data: &Vec<String>) -> usize {
	let mut total = 0;
	let mut processor = DataProcessor { lines: Vec::new() };

	for line in data {
		match line.as_str() {
			"" => total += processor.process_all().len(),
			x => processor.add(x)
		}
	}
	total += processor.process_all().len();

	return total;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn process_works() {
		let data: Vec<String> = "abc

a
b
c

ab
ac

a
a
a
a

b
".lines().map(|line| String::from(line)).collect();

		assert_eq!(process_data(&data), 11);
	}

	#[test]
	fn process_all_works() {
		let data: Vec<String> = "abc

a
b
c

ab
ac

a
a
a
a

b
".lines().map(|line| String::from(line)).collect();

		assert_eq!(process_all_data(&data), 6);
	}
}