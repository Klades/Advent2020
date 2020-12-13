pub fn part1(data: &Vec<String>) -> i32 {
	let mut ship = Ship::new();

	for line in data {
		let (action, value) = parse_instruction(line);
		ship.move_ship(action, value);
	}

	return ship.distance();
}

pub fn part2(data: &Vec<String>)-> i32 {
	let mut ship = Ship::new();

	for line in data {
		let (action, value) = parse_instruction(line);
		ship.move_waypoint(action, value);
	}

	return ship.distance();
}

fn parse_instruction(instruction: &str) -> (char, i32) {
	let action = instruction.chars().next().unwrap();
	let value = &instruction[1..].parse::<i32>().unwrap();

	return (action, *value);
}

struct Ship {
	facing: char,
	position: (i32, i32),
	waypoint: (i32, i32)
}

impl Ship {
	fn new() -> Ship {
		return Ship { facing: 'E', position: (0,0) , waypoint: (10,1) };
	}
	
	fn distance(&self) -> i32 {
		return self.position.0.abs() + self.position.1.abs();
	}

	fn move_ship(&mut self, direction: char, amount: i32) {
		let dir = if direction != 'F' { direction } else { self.facing };
		match dir {
			'N' => self.position.1 += amount,
			'S' => self.position.1 -= amount,
			'E' => self.position.0 += amount,
			'W' => self.position.0 -= amount,
			'R' => for _ in 0..(amount / 90) { self.facing = Ship::turn_right(self.facing) },
			'L' => for _ in 0..(amount / 90) { self.facing = Ship::turn_left(self.facing) }
			_ => panic!("bad action sent")
		}
	}

	fn move_waypoint(&mut self, direction: char, amount: i32) {
		match direction {
			'N' => self.waypoint.1 += amount,
			'S' => self.waypoint.1 -= amount,
			'E' => self.waypoint.0 += amount,
			'W' => self.waypoint.0 -= amount,
			'L' => for _ in 0..(amount / 90) { self.rotate_waypoint_left() },
			'R' => for _ in 0..(amount / 90) { self.rotate_waypoint_right() },
			'F' => self.move_to_waypoint(amount),
			_ => panic!("unexpected action")
		}
	}

	fn move_to_waypoint(&mut self, amount: i32) {
		self.position.0 += self.waypoint.0 * amount;
		self.position.1 += self.waypoint.1 * amount;
	}

	fn rotate_waypoint_left(&mut self) {
		let x = self.waypoint.0;
		let y = self.waypoint.1;
		self.waypoint.0 = -y;
		self.waypoint.1 = x;
	}

	fn rotate_waypoint_right(&mut self) {
		let x = self.waypoint.0;
		let y = self.waypoint.1;
		self.waypoint.0 = y;
		self.waypoint.1 = -x;
	}

	fn turn_left(facing: char) -> char {
		match facing {
			'N' => 'W',
			'W' => 'S',
			'S' => 'E',
			'E' => 'N',
			_ => panic!("bad facing found")
		}
	}

	fn turn_right(facing: char) -> char {
		match facing {
			'N' => 'E',
			'E' => 'S',
			'S' => 'W',
			'W' => 'N',
			_ => panic!("bad facing found")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ship_works() {
		let data: Vec<String> = "F10
N3
F7
R90
F11".lines().map(|line| String::from(line)).collect();

		let mut ship = Ship::new();
		for line in &data {
			let (action, amount) = parse_instruction(line);
			ship.move_ship(action, amount);
		}

		let distance = ship.distance();
		assert_eq!(distance.abs(), 25);
	}

	#[test]
	fn move_to_waypoint_works() {
		let mut ship = Ship::new();
		ship.move_waypoint('F', 10);

		assert_eq!(ship.position.0, 100);
		assert_eq!(ship.position.1, 10);
	}

	#[test]
	fn waypoint_rotation_works() {
		let mut ship = Ship::new();
		let waypoint = ship.waypoint.clone();

		ship.rotate_waypoint_right();
		ship.rotate_waypoint_right();
		ship.rotate_waypoint_right();
		ship.rotate_waypoint_right();

		assert_eq!(waypoint, ship.waypoint);
	}

	#[test]
	fn what_the_heck() {
		let mut ship = Ship::new();

		ship.move_waypoint('F', 10);
		assert_eq!(ship.position.0, 100); assert_eq!(ship.position.1, 10);

		ship.move_waypoint('N', 3);
		assert_eq!(ship.waypoint.0, 10); assert_eq!(ship.waypoint.1, 4);

		ship.move_waypoint('F', 7);
		assert_eq!(ship.position.0, 170); assert_eq!(ship.position.1, 38);

		ship.move_waypoint('R', 90);
		assert_eq!(ship.waypoint.0, 4); assert_eq!(ship.waypoint.1, -10);

		ship.move_waypoint('F', 11);
		assert_eq!(ship.position.0, 214); assert_eq!(ship.position.1, -72);
	}

	#[test]
	fn waypoint_ship_works() {
		let data: Vec<String> = "F10
N3
F7
R90
F11".lines().map(|line| String::from(line)).collect();

		let mut ship = Ship::new();
		for line in &data {
			let (action, amount) = parse_instruction(line);
			ship.move_waypoint(action, amount);
		}

		let distance = ship.distance();
		assert_eq!(distance.abs(), 286);
	}
}