struct TreeMap {
	data: Vec<String>,
	x_slope: usize,
	y_slope: usize
}

impl IntoIterator for TreeMap {
	type Item = char;
	type IntoIter = TreeMapIntoIterator;

	fn into_iter(self) -> Self::IntoIter {
		TreeMapIntoIterator { treemap: self, x: 0, y: 0 }
	}
}
struct TreeMapIntoIterator {
	treemap: TreeMap,
	x: usize,
	y: usize
}

impl Iterator for TreeMapIntoIterator {
	type Item = char;
	fn next(&mut self) -> Option<char> {
		self.x = (self.x + self.treemap.x_slope) % self.treemap.data[self.y].len();
		self.y = self.y + self.treemap.y_slope;

		if self.y >= self.treemap.data.len() {
			return None;
		}

		return self.treemap.data[self.y].chars().nth(self.x);
	}
}

pub fn part1(data: &Vec<String>) -> i32 {
	return count_trees(&data, 3, 1);
}

pub fn part2(data: &Vec<String>) -> i64 {
	let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	return slopes.iter().map(|pair| count_trees(&data, pair.0, pair.1) as i64).fold(1, |acc, x| acc * x);
}

fn count_trees(data: &Vec<String>, x_slope: usize, y_slope: usize) -> i32 {
	let map = TreeMap { data: data.clone(), x_slope: x_slope, y_slope: y_slope };

	return map.into_iter().filter(|c| *c == '#').count() as i32;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mapworks() {
		let data = vec!["..##.......".to_string(),
										"#...#...#..".to_string(),
										".#....#..#.".to_string(),
										"..#.#...#.#".to_string(),
										".#...##..#.".to_string(),
										"..#.##.....".to_string(),
										".#.#.#....#".to_string(),
										".#........#".to_string(),
										"#.##...#...".to_string(),
										"#...##....#".to_string(),
										".#..#...#.#".to_string(),
		];
		let map = TreeMap { data: data, x_slope: 3, y_slope: 1 };
		assert_eq!(map.into_iter().filter(|c| *c == '#').count(), 7);
	}
}