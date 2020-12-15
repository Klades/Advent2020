use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn part1(data: &str) -> usize {
	let numbers: Vec<usize> = data.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

	return number_game(&numbers, 2020);
}

pub fn part2(data: &str) -> usize {
	let numbers: Vec<usize> = data.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

	return number_game(&numbers, 30000000);
}

// I had a different solution before that did mostly the same with more code, but it took *40 seconds* to run so I tossed it after verifying it worked
// and copied in another, terser solution from the reddit thread
fn number_game(starting_numbers: &[usize], last_index: usize) -> usize {
	let mut numbers = starting_numbers[..starting_numbers.len() - 1].iter().copied().enumerate().map(|(index, num)| (num, index)).collect::<HashMap<_,_>>();

	(starting_numbers.len()..last_index).fold(*starting_numbers.last().unwrap(), |number, index| match numbers.entry(number) {
		Entry::Occupied(mut occ) => index - occ.insert(index - 1) - 1,
		Entry::Vacant(vac) => {
			vac.insert(index - 1);
			0
		}
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn number_game_works() {
		let data = vec![2,1,3];
		let start = std::time::Instant::now();
		assert_eq!(number_game(&data, 30000000), 3544142);
		println!("Time taken: {}", start.elapsed().as_millis());
	}
}