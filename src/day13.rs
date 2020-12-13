pub fn part1(data: &Vec<String>) -> usize {
	let departure_time = data[0].parse::<usize>().unwrap();
	let buses = parse_buses(&data[1]);

	let available_times = buses.iter().map(|bus| (calculate_bus_time(*bus, departure_time), *bus));
	let (actual_time, actual_bus) = available_times.min_by(|(time, _), (timeb, _)| time.cmp(timeb)).unwrap();

	return (actual_time - departure_time) * actual_bus;
}

// Nicked the part2 solution off of someone on reddit because after a few hours I wasn't having any luck or any fun
pub fn part2(data: &Vec<String>) -> i128 {
	let buses: Vec<Bus> = parse_relative_buses(&data[1]).iter().enumerate().filter(|(_, b)| b.is_some()).map(|(index, b)| Bus { id: b.unwrap(), offset: index as i128 }).collect();
	let offsets: Vec<i128> = buses.iter().map(|b| b.offset).collect();
	let values: Vec<i128> = buses.iter().map(|b| b.id).collect();
	
	values.iter().product::<i128>() - remainder(&offsets, &values).unwrap()
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
	if a == 0 {
		(b, 0, 1)
	}
	else {
		let (g, x, y) = egcd(b % a, a);
		(g, y - (b / a) * x, x)
	}
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
	let (g, x, _) = egcd(x, n);
	if g == 1 {
		Some((x % n + n) % n)
	}
	else { None }
}

fn remainder(offsets: &[i128], buses: &[i128]) -> Option<i128> {
	let total = buses.iter().product::<i128>();

	let mut sum = 0;
	for (&offset, &bus) in offsets.iter().zip(buses) {
		let x = total / bus;
		sum += offset * mod_inv(x, bus)? * x;
	}

	Some(sum % total)
}

#[derive(Copy, Clone)]
struct Bus {
	id: i128,
	offset: i128
}

fn calculate_bus_time(bus: usize, time: usize) -> usize {
	return (time / bus) * bus + bus;
}

fn parse_relative_buses(data: &str) -> Vec<Option<i128>> {
	let mut buses = vec![];
	for c in data.split(',') {
		match c {
			"x" => buses.push(None),
			x => buses.push(Some(x.parse::<i128>().unwrap()))
		}
	}

	return buses;
}

fn parse_buses(data: &str) -> Vec<usize> {
	return data.split(',').filter(|s| *s != "x").map(|n| n.parse::<usize>().unwrap()).collect();	
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn day13_part2_works() {
		let data = "1000
17,x,13,19".lines().map(|line| String::from(line)).collect();

		assert_eq!(part2(&data), 3417);
	}
}