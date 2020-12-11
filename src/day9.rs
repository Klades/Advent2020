use std::collections::VecDeque;

pub fn part1(data: &Vec<String>) -> i64 {
	let mut breaker = Breaker::new(25);

	for number in data.iter().map(|line| line.parse::<i64>().unwrap()) {
		if !breaker.process(number) { return number; }
	}
	// yes this should probably actually return Result or Option but whatever
	return -1;
}

pub fn part2(data: &Vec<String>) -> i64 {
	let target = part1(data);
	let numbers: Vec<i64> = data.iter().map(|line| line.parse::<i64>().unwrap()).collect();

	for i in 0..numbers.len() {
		let mut sum = numbers[i];
		for j in (i + 1)..numbers.len() {
			sum += numbers[j];

			if sum > target { continue; }
			if sum == target { return weakness_sum(&numbers[i..=j]); }
		}
	}

	return -1;
}

fn weakness_sum(numbers: &[i64]) -> i64 {
	let biggest = numbers.iter().max().unwrap();
	let smallest = numbers.iter().min().unwrap();

	return biggest + smallest;
}

struct Breaker {
	history: VecDeque<i64>,
	preamble_length: usize
}

impl Breaker {
	fn new(preamble_length: usize) -> Breaker {
		Breaker { history: VecDeque::<i64>::new(), preamble_length: preamble_length }
	}

	fn history_pairs(&self) -> Vec<i64> {
		let mut sums = vec!();
		for i in 0..self.history.len() {
			for j in (i + 1)..self.history.len() {
				sums.push(self.history[i] + self.history[j]);
			}	
		}

		return sums;
	}

	// Returns false if the number isn't valid
	// Returns true if the number is valid or part of the preamble
	fn process(&mut self, number: i64) -> bool {
		if self.history.len() < self.preamble_length {
			self.history.push_back(number);
			return true;
		}

		let sums = self.history_pairs();
		// There should be the proper number of numbers in the history at this point, so go ahead and push/pop
		// Doing it before we check the pairs so we don't have to save a return value
		self.history.push_back(number);
		self.history.pop_front();

		for sum in sums.iter() {
			if *sum == number { return true; } 
		}

		return false;
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn breaker_works() {
		let data: Vec<i64> = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576".lines().map(|line| line.parse::<i64>().unwrap()).collect();

		let mut breaker = Breaker::new(5);

		let result: i64 = {
			let mut x = 0;
			for number in data.iter() {
				if !breaker.process(*number) { x = *number; }
			}
			x
		};

		assert_eq!(result, 127);
	}
}