use std::{
  fs::{File, read_to_string},
  io::{prelude::*, BufReader},
	path::Path
};
use tokio::task;
use futures::future::join_all;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

struct DayResults {
	day:  i32,
	part1: String,
	part2: String
}

macro_rules! day_results {
	($day_mod:ident, $day_value:expr, $data:expr) => {
		DayResults { day: $day_value, part1: $day_mod::part1($data).to_string(), part2: $day_mod::part2($data).to_string() };
	}
}

#[tokio::main]
async fn main() {
	let mut day_futures = [
		task::spawn(async {
			let data = read_lines_to_numbers("data/day1.txt");
			return day_results!(day1, 1, &data);
			//return DayResults { day: 1, part1: day1::part1(&day1_data).to_string(), part2: day1::part2(&day1_data).to_string() };
		}),
		task::spawn(async {
			let data = read_lines("data/day2.txt");
			return day_results!(day2, 2, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day3.txt");
			return day_results!(day3, 3, &data);
		}),
		task::spawn(async {
			let data = read_chunk("data/day4.txt");
			return day_results!(day4, 4, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day5.txt");
			return day_results!(day5, 5, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day6.txt");
			return day_results!(day6, 6, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day7.txt");
			return day_results!(day7, 7, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day8.txt");
			return day_results!(day8, 8, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day9.txt");
			return day_results!(day9, 9, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day10.txt");
			return day_results!(day10, 10, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day11.txt");
			return day_results!(day11, 11, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day12.txt");
			return day_results!(day12, 12, &data);
		}),
		task::spawn(async {
			let data = read_lines("data/day13.txt");
			return day_results!(day13, 13, &data);
		})
	];

	let wrapped_results = join_all(day_futures.iter_mut()).await;
	let mut results: Vec<&DayResults> = wrapped_results.iter().flatten().collect();
	results.sort_by(|a, b| a.day.partial_cmp(&b.day).unwrap());

	for result in &results {
		println!("Day {} part 1 result: {}", result.day, result.part1);
		println!("Day {} part 2 result: {}", result.day, result.part2);
	}
}

fn read_lines_to_numbers(filename: impl AsRef<Path>) -> Vec<i32> {
	return read_lines(filename).iter().map(|line| line.parse::<i32>().unwrap()).collect();
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
	let file = File::open(filename).expect("no such file");
	let buf = BufReader::new(file);
	return buf.lines().map(|line| line.unwrap()).collect();
}

fn read_chunk(filename: impl AsRef<Path>) -> String {
	return read_to_string(filename).unwrap();
}
