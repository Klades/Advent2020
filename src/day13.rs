pub fn part1(data: &Vec<String>) -> i32 {
	let departure_time = data[0].parse::<i32>().unwrap();
	let buses = parse_buses(&data[1]);

	let available_times = buses.iter().map(|bus| (calculate_bus_time(*bus, departure_time), *bus));
	let (actual_time, actual_bus) = available_times.min_by(|(time, _), (timeb, _)| time.cmp(timeb)).unwrap();

	return (actual_time - departure_time) * actual_bus;
}

pub fn part2(data: &Vec<String>) -> i32 {
	return -1;
}

fn calculate_bus_time(bus: i32, time: i32) -> i32 {
	return (time / bus) * bus + bus;
}

fn parse_buses(data: &str) -> Vec<i32> {
	return data.split(',').filter(|s| *s != "x").map(|n| n.parse::<i32>().unwrap()).collect();	
}