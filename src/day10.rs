pub fn part1(data: &Vec<String>) -> i32 {
	return process_adapters(data);
}

pub fn part2(data: &Vec<String>) -> i64 {
	let values = process_data(data);
	let children = find_children(&values);
	let mut counts: Vec<i64> = vec![];
	counts.resize(values.len(), 0);
	*counts.last_mut().unwrap() = 1;

	for i in (0..values.len()).rev() {
		counts[i] = {
			let mut total = 0;
			for child in children[i].iter() {
				total += counts[*child as usize];
			}
			total
		};
	}

	return *counts.first().unwrap();
}

fn find_children(data: &Vec<i32>) -> Vec<Vec<i32>> {
	let mut children_list = vec![];
	for i in 0..data.len() {
		let mut single_list = vec![];

		for j in i..data.len() {
			if data[j] - data[i] <= 3 { single_list.push(j as i32); }
		}
		children_list.push(single_list);
	}

	return children_list;
}

fn process_data(data: &Vec<String>) -> Vec<i32> {
	let mut values: Vec<i32> = data.iter().map(|line| line.parse::<i32>().unwrap()).collect();
	values.push(0);
	values.sort();
	values.push(values.last().unwrap() + 3);

	return values;
}

fn process_adapters(data: &Vec<String>) -> i32 {
	let values = process_data(data);

	let mut ones = 0;
	let mut threes = 0;

	for i in 0..(values.len() - 1) {
		match values[i+1] - values[i] {
			1 => ones += 1,
			3 => threes += 1,
			_ => panic!("oh this isn't working at all!")
		}
	}

	return ones * threes;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process_adapters() {
		let data: Vec<String> = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3".lines().map(|line| String::from(line)).collect();

		assert_eq!(process_adapters(&data), 220);
	}

	#[test]
	fn test_counting_possibilities() {
		let data: Vec<String> = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3".lines().map(|line| String::from(line)).collect();

		assert_eq!(part2(&data), 19208);
	}
}