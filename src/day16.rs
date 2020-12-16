pub fn part1(data: &str) -> usize {
	if let [data_rules, _, data_tickets] = data.split("\n\n").collect::<Vec<&str>>().as_slice() {
		let rules = data_rules.lines().map(|rule| Rule::new(&rule)).collect::<Vec<_>>();
		let tickets = data_tickets.lines().skip(1).map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()));

		return tickets.map(|ticket| ticket.fold(0, |acc, x| acc + get_invalidation_score(x, &rules))).fold(0, |acc, x| acc + x);
	}

	panic!("failed to parse data");
}

pub fn part2(data: &str) -> usize {
	return 0;
}

fn get_invalidation_score(ticket_value: usize, rules: &Vec<Rule>) -> usize {
	if validate_ticket(ticket_value, rules) { return 0; }
	return ticket_value;
}

fn validate_ticket(ticket_value: usize, rules: &Vec<Rule>) -> bool {
	let debug = rules.iter().any(|rule| rule.validate(ticket_value));	
	return debug;
}

struct Rule {
	name: String,
	bounds: Vec<(usize, usize)>
}

impl Rule {
	fn new(data: &str) -> Rule {
		if let [class, range_strings] = data.split(": ").collect::<Vec<&str>>().as_slice()
		{
			let ranges = range_strings.split(" or ").map(|s| split_range(s)).collect::<Vec<_>>();
			return Rule { name: String::from(*class), bounds: ranges }
		}
		else { panic!("failed to parse rule"); }
	}

	fn validate(&self, value: usize) -> bool {
		for bound in &self.bounds {
			if value >= bound.0 && value <= bound.1 { return true; }
		}
	return false;
	}
}

fn split_range(range: &str) -> (usize, usize) {
	let mut split = range.split('-').map(|x| x.parse::<usize>().unwrap());
	(split.next().unwrap(), split.next().unwrap())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn day16_part1_works() {
		let data = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

		assert_eq!(part1(&data), 71);
	}
}