pub fn part1(data: &Vec<String>) -> i32 {
	let mut floor = Seating::new(data);

	while floor.process_floor_normal() {};

	return floor.count_seats();
}

pub fn part2(data: &Vec<String>) -> i32 {
	let mut floor = Seating::new(data);

	while floor.process_floor_ray() {};

	return floor.count_seats();
}

struct Seating {
	floor: Vec<Vec<char>>
}

impl Seating {
	fn new(data: &Vec<String>) -> Seating {
		let seats = data.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
		return Seating { floor: seats }
	}

	fn process_floor_normal(&mut self) -> bool {
		return self.process_floor(Seating::process_seat_normal);
	}

	fn process_floor_ray(&mut self) -> bool {
		return self.process_floor(Seating::process_seat_ray);
	}

	// Returns false if the floor didn't change
	fn process_floor<F>(&mut self, processor: F) -> bool where F: Fn(&mut Seating, usize, usize) -> char {
		let mut new_floor = self.floor.clone();

		for y in 0..self.floor.len() {
			for x in 0..self.floor[y].len() {
				new_floor[y][x] = processor(self, x, y);
			}
		}

		if self.floor == new_floor { return false; }

		self.floor = new_floor;
		return true;
	}

	fn count_seats(&self) -> i32 {
		return self.floor.iter().flatten().filter(|c| **c == '#').count() as i32;
	}

	fn process_seat_normal(&mut self, x: usize, y: usize) -> char {
		let occupied_neighbors = self.enumerate_neighbors(x as i32, y as i32).iter().filter(|seat| **seat == '#').count();

		return self.process_seat(x, y, occupied_neighbors);
	}

	fn process_seat_ray(&mut self, x: usize, y: usize) -> char {
		let occupied_sights = enumerate_pairs().iter().filter(|direction| self.scan_direction(x as i32, y as i32, direction)).count();

		return self.process_seat(x, y, occupied_sights);
	}

	// Returns true if scanning in a direction finds an occupied seat
	fn scan_direction(&self, mut x: i32, mut y: i32, direction: &(i32, i32)) -> bool {
		loop {
			x = x + direction.0;
			y = y + direction.1;

			let pair = (x, y);
			match self.pair_okay(&pair) {
				true => if self.floor[y as usize][x as usize] == '#' { return true; },
				false => return false
			}
		}
	}

	fn process_seat(&mut self, x: usize, y: usize, occupied_neighbors: usize) -> char {
		match self.floor[y][x] {
			'.' => '.',
			'#' if occupied_neighbors >= 4 => 'L',
			'L' if occupied_neighbors == 0 => '#',
			x => x
		}
	}

	fn enumerate_neighbors(&self, x: i32, y: i32) -> Vec<char> {
		let positions = [
			(x-1, y-1),
			(x, y-1),
			(x+1, y-1),
			(x-1, y),
			(x+1, y),
			(x-1, y+1),
			(x, y+1),
			(x+1, y+1)
		];

		return positions.iter().filter(|pair| self.pair_okay(&pair)).map(|pair| self.floor[pair.1 as usize][pair.0 as usize]).collect();
	}

	fn pair_okay(&self, pair: &(i32, i32)) -> bool {
		return pair.1 >= 0 && (pair.1 as usize) < self.floor.len() &&
			pair.0 >= 0 && (pair.0 as usize) < self.floor[pair.1 as usize].len(); 
	}
}

fn enumerate_pairs() -> Vec<(i32, i32)> {
	return vec![
		(-1, -1),
		(0, -1),
		(1, -1),
		(-1, 0),
		(1, 0),
		(-1, 1),
		(0, 1),
		(1, 1)
	];
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_vec_vec_compare() {
		let vector = vec![
			vec![1, 2, 3, 4],
			vec![3, 4, 3, 4]
		];

		let mut other_vec = vector.clone();

		assert_eq!(vector, other_vec);
		assert!(vector == other_vec);
		other_vec[0][3] = 9;
		assert_ne!(vector, other_vec);
		assert!(vector != other_vec);
	}

	#[test]
	fn test_floor_update() {
		let data: Vec<String> = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".lines().map(|line| String::from(line)).collect();

		let mut floor = Seating::new(&data);
		let result = floor.process_floor_normal();
		assert!(result);

		let second_data: Vec<String> = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##".lines().map(|line| String::from(line)).collect();

		let second_floor = Seating::new(&second_data);
		assert_eq!(floor.floor, second_floor.floor);
	}
}