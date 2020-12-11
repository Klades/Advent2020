use std::collections::HashMap;

pub fn part1(data: &Vec<String>) -> i32 {
	let rules: Vec<Rule> = data.iter().map(|line| parse_line(line)).collect();
	let bag_map = build_rulemap(&rules);

	return rules.iter().filter(|rule| can_contain_gold(rule, &bag_map)).count() as i32;
}

pub fn part2(data: &Vec<String>) -> i32 {
	let rules: Vec<Rule> = data.iter().map(|line| parse_line(line)).collect();
	let bag_map = build_rulemap(&rules);

	let gold_bag = bag_map.get("shiny gold").unwrap();

	// Have to remove the shiny gold bag from the count
	return count_bags(&gold_bag, &bag_map) - 1;
}

struct Rule {
	bag_name: String,
	contains: HashMap<String, i32>
}

fn can_contain_gold(bag: &Rule, bags: &HashMap<&str, &Rule>) -> bool {
	if bag.contains.contains_key("shiny gold") { return true; }

	for sub_bag in bag.contains.iter() {
		let bagstr = sub_bag.0.as_str();
		if bags.contains_key(bagstr) {
			if can_contain_gold(bags.get(sub_bag.0.as_str()).unwrap(), bags) { return true; }
		}
	}

	return false;
}

fn count_bags(bag: &Rule, bags: &HashMap<&str, &Rule>) -> i32 {
	// The bag itself counts as 1, so an empty bag is one bag
	if bag.contains.is_empty() { return 1; }

	let mut total = 1;
	for sub_bag in bag.contains.iter() {
		total += sub_bag.1 * count_bags(bags.get(sub_bag.0.as_str()).unwrap(), bags);
	}

	return total;
}

fn map_contains(line: &str) -> HashMap<String, i32> {
	let mut map = HashMap::<String, i32>::new();
	// contains "no other bags", return empty map
	if line.starts_with("no") { return map; }

	for bag in line.split(',') {
		let parts: Vec<&str> = bag.split_whitespace().collect();
		let name = String::from(parts[1]) + " " + parts[2];

		let count = parts[0].parse::<i32>().unwrap();

		map.insert(name, count);
	}

	return map;
}

fn parse_line(line: &str) -> Rule {
	let parts: Vec<&str> = line.split(" contain ").collect();
	let rules = map_contains(&parts[1]);
	let name = parts[0].strip_suffix(" bags").unwrap();

	return Rule { bag_name: String::from(name), contains: rules };
}

fn build_rulemap(rules: &Vec<Rule>) -> HashMap<&str, &Rule> {
	let mut map = HashMap::<&str, &Rule>::new();

	for rule in rules.iter() {
		map.insert(&rule.bag_name, &rule);
	}

	return map;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bags_works() {
		let lines: Vec<String> = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.".lines().map(|line| String::from(line)).collect();

		assert_eq!(part1(&lines), 4);
	}

	#[test]
	fn bag_counting_works()
	{
		let lines: Vec<String> = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.".lines().map(|line| String::from(line)).collect();

		assert_eq!(part2(&lines), 126);
	}
}