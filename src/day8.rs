pub fn part1(data: &Vec<String>) -> i64 {
	let mut console = Console::new(data);
	return console.run().0;
}

pub fn part2(data: &Vec<String>) -> i64 {
	let mut console = Console::new(data);

	for index in 0..console.program.len() {
		if console.program[index].operation != Operations::Acc {
			let (value, reason) = console.swap_and_run(index);
			if reason == TerminationReason::Completed { return value; }
		}
	}
	
	return 0;
}

fn swap_operation(instruction: &mut Instruction) -> bool {
	match instruction.operation {
		Operations::Nop => { instruction.operation = Operations::Jmp; return true }
		Operations::Jmp => { instruction.operation = Operations::Nop; return true }
		_ => return false
	}
}

fn parse_line(line: &str) -> Instruction {
	let parts: Vec<&str> = line.split_whitespace().collect();

	let operation = {
		match parts[0] {
			"nop" => Operations::Nop,
			"acc" => Operations::Acc,
			"jmp" => Operations::Jmp,
			_ => panic!("bad operation found?")
		}
	};

	let argument = parts[1].parse::<i64>().unwrap();

	return Instruction { operation: operation, value: argument, run_before: false };
}

fn parse_data(data: &Vec<String>) -> Vec<Instruction> {
	let instructions: Vec<Instruction> = data.iter().map(|line| parse_line(line)).collect();
	return instructions;
}

struct Console {
	accumulator: i64,
	pc: i64,
	program: Vec<Instruction>

}

impl Console {
	fn new(data: &Vec<String>) -> Console {
		let console = Console { accumulator: 0, pc: 0, program: parse_data(data) };
		return console;
	}

	fn reset(&mut self) {
		self.pc = 0;
		self.accumulator = 0;
		for instruction in &mut self.program {
			instruction.run_before = false;
		}
	}

	fn swap_and_run(&mut self, index_to_swap: usize) -> (i64, TerminationReason) {
		swap_operation(&mut self.program[index_to_swap]);
		let result = self.run();
		self.reset();
		swap_operation(&mut self.program[index_to_swap]);

		return result;
	}

	fn run(&mut self) -> (i64, TerminationReason) {
		while self.step().is_none() {}

		return (self.accumulator, self.step().unwrap());
	}

	fn step(&mut self) -> Option<TerminationReason> {
		if self.pc as usize >= self.program.len() { return Some(TerminationReason::Completed); }

		let instruction = &mut self.program[self.pc as usize];
		if instruction.run_before { return Some(TerminationReason::LoopDetected); }

		match instruction.operation {
			Operations::Acc => self.accumulator += instruction.value,
			Operations::Jmp => self.pc += instruction.value - 1,
			_ => ()

		}
		self.pc += 1;
		instruction.run_before = true;
		return None;
	}
}

#[derive(PartialEq, Eq)]
enum Operations {
	Acc,
	Jmp,
	Nop
}

#[derive(PartialEq, Eq)]
enum TerminationReason {
	Completed,
	LoopDetected
}

struct Instruction {
	operation: Operations,
	value: i64,
	run_before: bool
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn console_works() {
		let data: Vec<String> = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".lines().map(|line| String::from(line)).collect();

		assert_eq!(part1(&data), 5);
	}

	#[test]
	fn debug_console_works() {
		let data: Vec<String> = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".lines().map(|line| String::from(line)).collect();
		
		assert_eq!(part2(&data), 8);
	}
}