pub fn part1(data: &str) -> usize {
	if let [data_rules, _, data_tickets] = data.split("\n\n").collect::<Vec<&str>>().as_slice() {
		let rules = data_rules.lines().map(|rule| Rule::new(&rule)).collect::<Vec<_>>();
		let tickets = data_tickets.lines().skip(1).map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()));

		return tickets.map(|ticket| ticket.fold(0, |acc, x| acc + get_invalidation_score(x, &rules))).fold(0, |acc, x| acc + x);
	}

	panic!("failed to parse data");
}

pub fn part2(data: &str) -> usize {
	if let [data_rules, my_ticket_str, data_tickets] = data.split("\n\n").collect::<Vec<&str>>().as_slice() {
		let rules = data_rules.lines().map(|rule| Rule::new(&rule)).collect::<Vec<_>>();
		let my_ticket = my_ticket_str.lines().skip(1).map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()).next().unwrap();
		let tickets = data_tickets.lines().skip(1).map(|line| parse_ticket(line)).filter(|t| validate_whole_ticket(t, &rules)).collect::<Vec<_>>();

		let fields = map_fields(&tickets, &rules);
		let mut assigned = assign_fields(&fields).unwrap();
		assigned.sort_by(|a, b| a.id.cmp(&b.id));

		let field_names = assigned.iter().map(|field| rules[field.possible_rules[0]].name.clone()).collect::<Vec<_>>();
		let departure_fields = field_names.iter().zip(my_ticket).filter(|(name, _)| name.starts_with("departure")).fold(1, |acc, (_, value)| acc * value);

		return departure_fields;
	}
	return 0;
}

fn assign_fields(field_slice: &[Field]) -> Option<Vec<Field>> {
	let mut fields = field_slice.to_vec();
	fields.sort_by(|a, b| a.possible_rules.len().cmp(&b.possible_rules.len()));
	let field = &fields[0];

	if field.possible_rules.len() == 0 { return None; }

	// If there's only one thing in the list, it _should_ only have one possible rule left for it, anyway
	if fields.len() == 1 { return Some(fields); }

	for rule in &field.possible_rules {
		let stripped_fields = fields[1..].iter().map(|f| without_rule(&f, *rule)).collect::<Vec<_>>();
		if let Some(mut field_list) = assign_fields(&stripped_fields) {
			field_list.push(Field { id: field.id, possible_rules: vec![*rule] });
			return Some(field_list);
		}
	}

	// Exhausted all possible rules for this branch
	return None;
}

fn without_rule(field: &Field, rule: usize) -> Field {
	Field { id: field.id, possible_rules: field.possible_rules.iter().copied().filter(|r| *r != rule).collect() }
}

fn map_fields(tickets: &Vec<Vec<usize>>, rules: &Vec<Rule>) -> Vec<Field> {
	let ticket_fields = (0..rules.len()).map(|i| tickets.iter().map(|t| t[i]).collect::<Vec<_>>()).collect::<Vec<_>>();
	ticket_fields.iter().enumerate().map(|(i, values)| Field::new(i, &scan_rules(values, &rules))).collect::<Vec<_>>()
}

fn scan_rules(values: &Vec<usize>, rules: &Vec<Rule>) -> Vec<usize> {
	rules.iter().enumerate().filter(|(_, r)| values.iter().all(|&v| r.validate(v))).map(|(i, _)| i).collect()
}

fn parse_ticket(ticket: &str) -> Vec<usize> {
	ticket.split(',').map(|i| i.parse::<usize>().unwrap()).collect()
}

fn get_invalidation_score(ticket_value: usize, rules: &Vec<Rule>) -> usize {
	if validate_ticket(ticket_value, rules) { return 0; }
	return ticket_value;
}

fn validate_whole_ticket(ticket: &Vec<usize>, rules: &Vec<Rule>) -> bool {
	ticket.iter().all(|t| validate_ticket(*t, rules))
}

fn validate_ticket(ticket_value: usize, rules: &Vec<Rule>) -> bool {
	rules.iter().any(|rule| rule.validate(ticket_value))
}

#[derive(Clone, Debug)]
struct Field {
	id: usize,
	possible_rules: Vec<usize>
}

impl Field {
	fn new(id: usize, rules: &Vec<usize>) -> Field {
		Field { id: id, possible_rules: rules.clone() }
	}
}

#[derive(Clone)]
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

	#[test]
	fn day16_part2_works() {
		let data = "departure class: 0-1 or 4-19
row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

		assert_eq!(part2(&data), 11 * 13);
	}
}