use std::collections::HashMap;

pub fn part1(data: &Vec<String>) -> usize {
	let mut seats = Seating::new(data);
	while seats.step(4) {}
	return seats.count();
}

pub fn part2(data: &Vec<String>) -> i32 {
	return -1;
}

struct Seating {
	seats: Vec<char>,
	neighbors: HashMap<usize, Vec<usize>>
}

impl Seating {
	fn new(data: &Vec<String>) -> Seating {
		let height = data.len();
		let width = data[0].len();
		let chars = data.iter().map(|line| line.chars()).flatten().collect();

		let mut neighbors = HashMap::new();
		for x in 0..width {
			for y in 0..height {
				neighbors.insert(coords(x, y), enumerate_neighbors(x, y));
			}
		}

		return Seating { seats: chars, neighbors: neighbors };
	}

	fn step(&mut self, neighbor_limit: usize) -> bool {
		let mut new_seats = vec![];

		for (index, s) in self.seats.iter().enumerate() {
			new_seats.push(self.process_seat(*s, &self.neighbors[&index], neighbor_limit));
		}

		if new_seats == self.seats { return false; }

		self.seats = new_seats;
		return true;
	}

	fn count(&self) -> usize {
		return self.seats.iter().filter(|c| **c == '#').count();
	}

	fn process_seat(&self, seat: char, neighbors: &Vec<usize>, limit: usize) -> char {
		let neighbor_count = neighbors.iter().map(|index| self.seats[*index]).filter(|c| *c == '#').count();
		let crowded = neighbor_count >= limit;
		let empty = neighbor_count == 0;

		match seat {
			'#' if crowded => return 'L',
			'L' if empty => return '#',
			x => return x
		}
	}
}


fn coords(x: usize, y: usize) -> usize {
	return y * 10 + x;
}

fn enumerate_neighbors(x: usize, y: usize) -> Vec<usize> {
	let x_copy = x as i32;
	let y_copy = y as i32;

	let mut neighbor_coords = vec![];
	for i in (x_copy - 1)..(x_copy + 1) {
		for j in (y_copy - 1)..(y_copy + 1) {
			if i >= 0 && j >= 0 && ((i as usize) != x || (j as usize) != y) {
				neighbor_coords.push(coords(i as usize, j as usize));
			}
		}
	}

	return neighbor_coords;
}