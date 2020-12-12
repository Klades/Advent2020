use std::fmt;

pub fn part1(data: &Vec<String>) -> usize {
	let mut seats = Seating::new(data, enumerate_neighbors);
	while seats.step(4) {}
	return seats.count();
}

pub fn part2(data: &Vec<String>) -> usize {
	let mut seats = Seating::new(data, enumerate_sight_neighbors);
	while seats.step(5) {}
	return seats.count();
}

struct Seating {
	seats: Vec<Space>,
}

struct Space {
	value: char,
	neighbors: Vec<usize>
}

impl Space {
	fn new(value: char, neighbors: Vec<usize>) -> Space {
		match value {
			'.' => Space { value: value, neighbors: vec![] },
			x => Space { value: x, neighbors: neighbors }
		}
	}
}

impl PartialEq<Space> for Space {
	fn eq(&self, other: &Space) -> bool {
		return self.value == other.value;
	}
}

impl PartialEq<char> for Space {
	fn eq(&self, other: &char) -> bool {
		return self.value == *other;
	}
}

impl fmt::Debug for Space {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Space").field("value", &self.value).finish()
	}
}

impl Seating {
	fn new<F>(data: &Vec<String>, neighbor_finder: F) -> Seating where F: Fn(usize, usize, usize, usize, &Vec<char>) -> Vec<usize> {
		let height = data.len();
		let width = data[0].len();
		let chars: Vec<char> = data.iter().map(|line| line.chars()).flatten().collect();

		let mut neighbor_vec = vec![];
		for y in 0..height {
			for x in 0..width {
				neighbor_vec.push(neighbor_finder(x, y, width, height, &chars));
			}
		}

		let spaces: Vec<Space> = chars.iter().zip(neighbor_vec).map(|(c, n)| Space::new(*c, n)).collect();
		return Seating { seats: spaces };
	}

	fn step(&mut self, neighbor_limit: usize) -> bool {
		let mut new_seats: Vec<char> = vec![];

		for seat in &self.seats {
			let new_value = self.process_seat(&seat, neighbor_limit);
			new_seats.push(new_value);
		}

		if self.seats == new_seats { return false; }

		// Well this sucks
		for i in 0..new_seats.len() {
			self.seats[i].value = new_seats[i];
		}

		return true;
	}

	fn count(&self) -> usize {
		return self.seats.iter().filter(|c| **c == '#').count();
	}

	fn process_seat(&self, seat: &Space, limit: usize) -> char {
		let neighbor_count = seat.neighbors.iter().map(|index| &self.seats[*index]).filter(|c| c.value == '#').count();
		let new_value = match_seat(seat.value, neighbor_count, limit);
		return new_value;
	}
}

fn match_seat(seat: char, neighbors: usize, limit: usize) -> char {
	let crowded = neighbors >= limit;
	let empty = neighbors == 0;

	match seat {
		'#' if crowded => return 'L',
		'L' if empty => return '#',
		x => return x
	}
}

fn coords(x: usize, y: usize, coord_factor: usize) -> usize {
	return y * coord_factor + x;
}

fn enumerate_neighbors(x: usize, y: usize, maxwidth: usize, maxheight: usize, _: &Vec<char>) -> Vec<usize> {
	let x_copy = x as i32;
	let y_copy = y as i32;

	let mut neighbor_coords = vec![];
	for i in (x_copy - 1)..=(x_copy + 1) {
		for j in (y_copy - 1)..=(y_copy + 1) {
			if pair_okay(i, j, x, y, maxwidth, maxheight) {
				neighbor_coords.push(coords(i as usize, j as usize, maxwidth));
			}
		}
	}
	return neighbor_coords;
}

fn enumerate_sight_neighbors(x: usize, y: usize, maxwidth: usize, maxheight: usize, floor: &Vec<char>) -> Vec<usize> {
	let mut neighbors = vec![];

	for x_move in &[-1, 0, 1] {
		for y_move in &[-1, 0, 1] {
			let mut x_here = x as i32;
			let mut y_here = y as i32;

			if *x_move == 0 && *y_move == 0 { continue; }

			loop {
				x_here += x_move;
				y_here += y_move;

				if !pair_okay(x_here, y_here, x, y, maxwidth, maxheight) { break; }
				let here = coords(x_here as usize, y_here as usize, maxwidth);

				if floor[here] != '.' { 
					neighbors.push(here);
					break;
				}
			}
		}
	}
	return neighbors;
}

fn pair_okay(x: i32, y: i32, original_x: usize, original_y: usize, max_x: usize, max_y: usize) -> bool {
	let x_okay = x >= 0 && (x as usize) < max_x;
	let y_okay = y >= 0 && (y as usize) < max_y;

	let not_origin = (x as usize) != original_x || (y as usize) != original_y;

	return x_okay && y_okay && not_origin;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn process_seat_works() {
		assert_eq!(match_seat('#', 4, 4), 'L');
		assert_eq!(match_seat('#', 0, 4), '#');
		assert_eq!(match_seat('L', 0, 4), '#');
		assert_eq!(match_seat('L', 4, 4), 'L');
		assert_eq!(match_seat('.', 0, 1), '.');
	}

	#[test]
	fn step_works_without_sight() {
		let data = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##".lines().map(|line| String::from(line)).collect();

		let mut seats = Seating::new(&data, enumerate_neighbors);
		seats.step(4);

		let stepped_data = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##".lines().map(|line| String::from(line)).collect();

		let stepped_seats = Seating::new(&stepped_data, enumerate_neighbors);

		assert_eq!(seats.count(), stepped_seats.count());
		assert_eq!(seats.seats, stepped_seats.seats);
	}

	#[test]
	fn step_works_with_sight() {
		let data = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##".lines().map(|line| String::from(line)).collect();

		let mut seats = Seating::new(&data, enumerate_sight_neighbors);
		seats.step(5);

		let stepped_data = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#".lines().map(|line| String::from(line)).collect();

		let stepped_seats = Seating::new(&stepped_data, enumerate_sight_neighbors);

		assert_eq!(seats.count(), stepped_seats.count());
		assert_eq!(seats.seats, stepped_seats.seats);

	}
}