pub fn part1(data: &[i32]) -> i32 {
	return search(2020, data[0], &data[1..]).unwrap();
}

pub fn part2(data: &[i32]) -> i32 {
	return search_three(data[0], &data[1..]).unwrap();
}

fn search_three(base: i32, rest: &[i32]) -> Option<i32> {
	if rest.len() < 2 {
		return None;
	}

	match search(2020 - base, rest[0], &rest[1..]) {
		Some(value) => return Some(value * base),
		None => return search_three(rest[0], &rest[1..])
	}
}

fn search(target: i32, base: i32, rest: &[i32]) -> Option<i32> {
  for other in rest.iter() {
    if base + other == target {
      return Some(base * other);
    }
  }

  if rest.len() <= 1 {
    return None;
  }
  return search(target, rest[0], &rest[1..]);
}