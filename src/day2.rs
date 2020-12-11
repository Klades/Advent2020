struct Password {
	upper_limit: i32,
	lower_limit: i32,
	limited_char: char,
	password: String
}

fn make_password(data: &str) -> Password {
	let fields: Vec<&str> = data.split_ascii_whitespace().collect();
	let limits: Vec<i32> = fields[0].split('-').map(|value| value.parse::<i32>().unwrap()).collect();
	let lower_limit = limits[0];
	let upper_limit = limits[1];
	let limited_char = fields[1].chars().nth(0).unwrap();
	
	return Password { upper_limit: upper_limit, lower_limit: lower_limit, limited_char: limited_char, password: fields[2].to_string() };
}

fn check_password(password: &Password) -> bool {
	let count = password.password.matches(password.limited_char).count() as i32;
	return count >= password.lower_limit && count <= password.upper_limit;
}

fn check_password_again(password: &Password) -> bool {
	let first_char = password.password.chars().nth((password.lower_limit - 1) as usize).unwrap();
	let second_char = password.password.chars().nth((password.upper_limit - 1) as usize).unwrap();

	let count = (first_char == password.limited_char) as i32 + (second_char == password.limited_char) as i32;

	return count == 1;
}

pub fn part1(data: &Vec<String>) -> i32 {
	return check(data, |password| check_password(password));
}

pub fn part2(data: &Vec<String>) -> i32 {
	return check(data, |password| check_password_again(password));
}

fn check<F>(data: &Vec<String>, f: F) -> i32 where F: FnMut(&&Password) -> bool {
	let passwords: Vec<Password> = data.iter().map(|line| make_password(line)).collect();
	let good_passwords = passwords.iter().filter(f).count() as i32;

	return good_passwords;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn passwords_build() {
		let password = make_password("1-3 a: abcde");
		assert_eq!(password.upper_limit, 3);
		assert_eq!(password.lower_limit, 1);
		assert_eq!(password.limited_char, 'a');
		assert_eq!(password.password, "abcde");
	}

	#[test]
	fn day2part1works() {
		let passwords = vec!["1-3 a: abcde".to_string(), "1-3 b: cdefg".to_string(), "2-9 c: ccccccccc".to_string()];
		assert_eq!(part1(&passwords), 2);
	}
}