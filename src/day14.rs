use std::collections::HashMap;

pub fn part1(data: &Vec<String>) -> u128 {
	let mut system = System::new();

	for line in data.iter() {
		let cmd = parse_command(line);
		system.run_command(cmd);
	}

	return system.memory_sum();
}

pub fn part2(data: &Vec<String>) -> u128 {
	let mut system = System::new();

	for line in data.iter() {
		let cmd = parse_command_floating(line);
		system.run_command(cmd);
	}

	return system.memory_sum();
}

struct System {
	mem: HashMap<usize, u64>,
	and_mask: u64,
	or_mask: u64,
	floating_mask: FloatingMask
}

impl System {
	fn new() -> System {
		System { mem: HashMap::<usize, u64>::new(), and_mask: 0, or_mask: u64::max_value(), floating_mask: FloatingMask::new("0")}
	}

	fn run_command(&mut self, cmd: Command) {
		match cmd {
			Command::Mask(x) => self.change_masks(x),
			Command::Mem(i, v) => {
				self.mem.insert(i, self.apply_mask(v));
			},
			Command::FloatingMask(m) => self.floating_mask = m,
			Command::FloatingMem(i, v) => {
				for (and_mask, or_mask) in self.floating_mask.iter_mut() {
					let address = (i & and_mask as usize) | or_mask as usize;
					self.mem.insert(address, v);
				}
			}
		}
	}

	fn change_masks(&mut self, mask: &str) {
		let masks = update_masks(mask);
		self.and_mask = masks.0;
		self.or_mask = masks.1;
	}

	fn apply_mask(&self, value: u64) -> u64 {
		return (value | self.or_mask) & self.and_mask;
	}

	fn memory_sum(&self) -> u128 {
		return self.mem.iter().fold(0, |acc, x| acc + *x.1 as u128);
	}
}
#[derive(Debug)]
struct FloatingBit {
	index: usize,
	value: u64
}

struct FloatingMask {
	mask: u64,
	floaters: Vec<FloatingBit>
}

impl FloatingMask {
	fn new(mask: &str) -> FloatingMask { 
		let mut bits = 0;
		let mut floaters = vec![];
		// Need to "reverse" the index of each char when storing the bits so the bit bashing works correctly
		let last_index = mask.len() - 1;
		for (index, c) in mask.chars().enumerate() {
			match c {
				'1' => bits = (bits << 1) + 1,
				'0' => bits = bits << 1,
				'X' => { bits = bits << 1; floaters.push(FloatingBit { index: last_index - index, value: 0 }) },
				_ => panic!("unexpected input")
			}
		}
		floaters.reverse();
		return FloatingMask { mask: bits, floaters: floaters } 
	}
	
	fn iter_mut(&mut self) -> FloatingMaskIterator {
		return FloatingMaskIterator::new(self);
	}

	fn collect_mask(&self) -> (u64, u64) {
		let mut and_mask = u64::max_value();
		let mut or_mask = self.mask;

		for floater in self.floaters.iter() {
			match floater.value {
				0 => and_mask -= 1 << floater.index,
				1 => or_mask |= 1 << floater.index,
				_ => panic!("unexpected floater bit value")
			}
		}

		(and_mask, or_mask)
	}
}

struct FloatingMaskIterator<'a> {
	mask: &'a mut FloatingMask,
	done: bool
}

impl<'a> FloatingMaskIterator<'a> {
	fn new(mask: &mut FloatingMask) -> FloatingMaskIterator {
		FloatingMaskIterator { mask: mask, done: false }
	}
}

impl<'a> Iterator for FloatingMaskIterator<'a> {
	type Item = (u64, u64);

	fn next(&mut self) -> Option<(u64, u64)> {
		if self.done { return None }
		// Grab the currently stored mask value for later return
		let current_mask = self.mask.collect_mask();

		// Update the mask; self.done should be set to true if current_mask is all 1s on the floaters
		let mut it = self.mask.floaters.iter_mut();
		loop {
			let bit = match it.next() {
				None => { self.done = true; break; }
				Some(x) => x
			};
			if bit.value == 0 { bit.value = 1; break; }
			else { bit.value = 0; }
		}

		return Some(current_mask);
	}
}

fn update_masks(mask: &str) -> (u64, u64) {
	let (mut and_mask, mut or_mask) = (0, 0);

	for b in mask.chars() {
		match b {
			'X' => {
				and_mask = (and_mask << 1) + 1;
				or_mask = or_mask << 1;
			},
			'1' => {
				and_mask = (and_mask << 1) + 1;
				or_mask = (or_mask << 1) + 1;
			},
			'0' => {
				and_mask = and_mask << 1;
				or_mask = or_mask << 1;
			},
			_ => panic!("unexpected mask bit")
		}
	}

	return (and_mask, or_mask);
}

enum Command<'a> {
	Mask(&'a str),
	FloatingMask(FloatingMask),
	Mem(usize, u64),
	FloatingMem(usize, u64)
}

fn parse_command(line: &str) -> Command {
	let parts: Vec<&str> = line.split(" = ").collect();
	let value = parts[1];
	match parts[0] {
		"mask" => return Command::Mask(value),
		x => return Command::Mem(strip_mem(x), value.parse::<u64>().unwrap())
	}
}

fn parse_command_floating(line: &str) -> Command {
	let parts: Vec<&str> = line.split(" = ").collect();
	let value = parts[1];
	match parts[0] {
		"mask" => return Command::FloatingMask(FloatingMask::new(value)),
		x => return Command::FloatingMem(strip_mem(x), value.parse::<u64>().unwrap())
	}
}

fn strip_mem(tag: &str) -> usize {
	return tag.strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse::<usize>().unwrap();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn floating_calculations_work() {
		let data = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1".lines().map(|line| String::from(line)).collect();

		assert_eq!(part2(&data), 208);
	}
}