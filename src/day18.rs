use std::collections::{
	HashMap,
	VecDeque
};

pub fn part1(data: &Vec<String>) -> i64 {
	let mut priorities = HashMap::new();
	priorities.insert(Token::Add, 1);
	priorities.insert(Token::Mul, 1);
	let tokens = data.iter().map(move |line| process_line(line, &priorities));
	tokens.map(|line| calculate_line(line)).fold(0, |acc, x| acc + x)
}

pub fn part2(data: &Vec<String>) -> i64 {
	let mut priorities = HashMap::new();
	priorities.insert(Token::Add, 1);
	priorities.insert(Token::Mul, 2);
	let tokens = data.iter().map(|line| process_line(line, &priorities));
	tokens.map(|line| calculate_line(line)).fold(0, |acc, x| acc + x)
}

#[derive(PartialEq, Hash, Eq)]
enum Token {
	Add,
	Mul,
	Num(i64),
	OpenParen,
	CloseParen
}

fn calculate_line(mut line: VecDeque<Token>) -> i64 {
	let mut operands = vec![];

	while !line.is_empty() {
		match line.pop_front().unwrap() {
			Token::Num(x) => operands.push(x),
			Token::Add => { let x = arithmetic(&mut operands, |a, b| a + b); operands.push(x); },
			Token::Mul => { let x = arithmetic(&mut operands, |a, b| a * b); operands.push(x); },
			_ => panic!("someone left a paren on the output queue")
		}
	}

	return operands.pop().unwrap();
}


fn arithmetic<F>(stack: &mut Vec<i64>, func: F) -> i64 where F: Fn(i64, i64) -> i64 {
	let a = stack.pop();
	let b = stack.pop();

	match (a, b) {
		(Some(a), Some(b)) => func(a, b),
		_ => panic!("ran out of numbers!!")
	}
}

fn process_line(line: &str, priorities: &HashMap<Token, i32>) -> VecDeque<Token> {
	let tokens = parse_tokens(line);
	let mut output = VecDeque::new();
	let mut stack = vec![];
	for token in tokens {
		match token {
			x @ Token::Num(_) => output.push_back(x),
			x => process_operator(x, &mut stack, &mut output, &priorities)
		}
	}

	while !stack.is_empty()
	{
		output.push_back(stack.pop().unwrap());
	}

	return output;
}

fn process_operator(token: Token, stack: &mut Vec<Token>, output: &mut VecDeque<Token>, priorities: &HashMap<Token, i32>) {
	if stack.is_empty() {
		stack.push(token);
		return;
	}

	match token {
		Token::CloseParen => {
			while stack.last().unwrap() != &Token::OpenParen { output.push_back(stack.pop().unwrap()); }
			stack.pop(); // Get rid of that open paren
		},
		Token::OpenParen => stack.push(token),
		Token::Num(_) => panic!("confused a number and an operator somehow??"),
		x => {
			loop {
				if stack.is_empty() { stack.push(x); return; }

				let last = stack.last().unwrap();
				let prioritized = priorities.get(last).or(Some(&0)).unwrap() <= priorities.get(&x).unwrap();

				if !prioritized || last == &Token::OpenParen {
					stack.push(x);
					return;
				}

				output.push_back(stack.pop().unwrap());
			}
		}
	}
}

fn parse_tokens(line: &str) -> Vec<Token> {
	let mut output = vec![];
	for s in line.chars() {
		match s {
			' ' => continue,
			'+' => output.push(Token::Add),
			'*' => output.push(Token::Mul),
			'(' => output.push(Token::OpenParen),
			')' => output.push(Token::CloseParen),
			x => output.push(Token::Num(String::from(x).parse::<i64>().unwrap()))
		}
	}

	return output;
}