use std::collections::HashMap;

#[allow(unreachable_code, unused_variables)]
pub fn part1(data: &Vec<String>) -> usize {
	return 0;
	let mut field = Field::new_3d(data);

	for _ in 0..6 {
		field.process_field();
	}

	return field.count();
}

pub fn part2(data: &Vec<String>) -> usize {
	let mut field = Field::new_4d(data);

	for _ in 0..6 {
		field.process_field();
	}

	return field.count();
}

type CubeSpace = HashMap<(i32, i32, i32, Option<i32>), bool>;

struct Field {
	cubes: CubeSpace,
	borders: Bounds
}

struct Bounds {
	x: (i32, i32),
	y: (i32, i32),
	z: (i32, i32),
	w: Option<(i32, i32)>
}

impl Bounds {
	fn iter(&self) -> BoundsIter {
		BoundsIter { bounds: self, x: self.x.0, y: self.y.0, z: self.z.0, w: if let Some(w) = self.w { Some(w.0) } else { None } }
	}

	fn grow(&mut self) {
		self.x.0 -= 1;
		self.x.1 += 1;
		self.y.0 -= 1;
		self.y.1 += 1;
		self.z.0 -= 1;
		self.z.1 += 1;
		if let Some(w) = &mut self.w {
			w.0 -= 1;
			w.1 += 1;
		}
	}
}

struct BoundsIter<'a> {
	bounds: &'a Bounds,
	x: i32,
	y: i32,
	z: i32,
	w: Option<i32>
}

impl<'a> Iterator for BoundsIter<'a> {
	type Item = (i32, i32, i32, Option<i32>);
	fn next(&mut self) -> Option<Self::Item> {
		let value = (self.x, self.y, self.z, self.w);

		self.z += 1;
		if self.z > self.bounds.z.1 {
			self.z = self.bounds.z.0;

			self.y += 1;
			if self.y > self.bounds.y.1 {
				self.y = self.bounds.y.0;

				self.x += 1;
				if self.x > self.bounds.x.1 {
					match &mut self.w {
						None => return None,
						Some(w) => {
							self.x = self.bounds.x.0;

							*w += 1;
							if *w > self.bounds.w.unwrap().1 {
								return None;
							}
						}
					}
					return None;
				}
			}
		}

		return Some(value);
	}
}

impl Field {
	fn new_3d(data: &Vec<String>) -> Field {
		let cubes = Field::initialize(data, None);
		let bounds = Bounds { x: (-1, data[0].len() as i32), y: (-1, data.len() as i32), z: (-1, 1), w: None };
		Field { cubes: cubes, borders: bounds }
	}

	fn new_4d(data: &Vec<String>) -> Field {
		let cubes = Field::initialize(data, Some(0));
		let bounds = Bounds { x: (-1, data[0].len() as i32), y: (-1, data.len() as i32), z: (-1, 1), w: Some((-1, 1)) };
		Field { cubes: cubes, borders: bounds }
	}

	fn initialize(data: &Vec<String>, w_value: Option<i32>) -> CubeSpace {
		let mut cubes = HashMap::new();
		for (y, line) in data.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				cubes.insert((x as i32, y as i32, 0, w_value), c == '#');
			}
		}
		return cubes;
	}

	fn get_cube(&self, x: i32, y: i32, z: i32, w: Option<i32>) -> bool {
		match self.cubes.get(&(x, y, z, w)) {
			None => return false,
			Some(x) => return *x
		}
	}
	
	fn process_cube(&self, x_0: i32, y_0: i32, z_0: i32, w_0: Option<i32>) -> bool {
		let mut count = 0;
		for x in x_0-1..=x_0+1 {
			for y in y_0-1..=y_0+1 {
				for z in z_0-1..=z_0+1 {
					match w_0 {
						None => {
							if x != x_0 || y != y_0 || z != z_0 {
								if self.get_cube(x, y, z, None) { count += 1; }
							}
						},
						Some(w0) => {
							for w in w0-1..=w0+1 {
								if x != x_0 || y != y_0 || z != z_0 || w != w0 {
									if self.get_cube(x, y, z, Some(w)) { count += 1; }
								}
							}
						}
					}
				}
			}
		}

		let alive = self.get_cube(x_0, y_0, z_0, w_0);
		match count {
			2|3 if alive => return true,
			3 if !alive => return true,
			_ => return false
		}
	}

	fn process_field(&mut self) {
		let mut new_space: CubeSpace = HashMap::new();

		for (x, y, z, w) in self.borders.iter() {
			new_space.insert((x, y, z, w), self.process_cube(x, y, z, w));
		}

		self.cubes = new_space;
		self.borders.grow();
	}

	fn count(&self) -> usize {
		return self.cubes.values().filter(|&b| *b).count()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn day17_works() {
		let data = ".#.
..#
###".lines().map(|line| String::from(line)).collect::<Vec<_>>();

		assert_eq!(part1(&data), 112);
		assert_eq!(part2(&data), 848);
	}
}