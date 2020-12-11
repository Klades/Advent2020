use std::collections::HashMap;

struct Passport<'a> {
	map: HashMap<&'a str, &'a str>
}

fn keymatch<T: std::str::FromStr>(map: &HashMap<&str, &str>, key: &str) -> Option<T> {
	match map.get(key) {
		None => return None,
		Some(value) => match value.parse::<T>() {
			Ok(number) => return Some(number),
			_ => return None
		}
	}
}

impl Passport<'_> {
	fn byr(&self) -> Option<i32> {
		match keymatch(&self.map, "byr") {
			Some(value) if value >= 1920 && value <= 2002 => return Some(value),
			_ => return None
		}
	}

	fn iyr(&self) -> Option<i32> {
		match keymatch(&self.map, "iyr") {
			Some(value) if value >= 2010 && value <= 2020 => return Some(value),
			_ => return None
		}
	}

	fn eyr(&self) -> Option<i32> {
		match keymatch(&self.map, "eyr") {
			Some(value) if value >= 2020 && value <= 2030 => return Some(value),
			_ => return None
		}
	}

	fn hgt(&self) -> Option<(i32, &str)> {
		let raw: &str = { 
			match self.map.get("hgt") {
				None => return None,
				Some(value) => value
			}
		};
		let index = raw.len() - 2;
		let value = {
			match raw[..index].parse::<i32>() {
				Ok(number) => number,
				_ => return None
			}
		};
		let suffix = {
			match &raw[index..] {
				"in" if value >= 59 && value <= 76 => "in",
				"cm" if value >= 150 && value <= 193 => "cm",
				_ => return None
			}
		};

		return Some((value, suffix))
	}

	fn hcl(&self) -> Option<&str> {
		let value = {
			match self.map.get("hcl") {
				None => return None,
				Some(line) => line
			}
		};

		if value.chars().nth(0).unwrap() == '#' && value.len() == 7 {
			return Some(value)
		}

		return None
	}

	fn ecl(&self) -> Option<&str> {
		match self.map.get("ecl") {
			Some(&"amb") => return Some("amb"),	
			Some(&"blu") => return Some("blu"),
			Some(&"brn") => return Some("brn"),
			Some(&"gry") => return Some("gry"),
			Some(&"grn") => return Some("grn"),
			Some(&"hzl") => return Some("hzl"),
			Some(&"oth") => return Some("oth"),
			_ => return None
		}
	}

	fn pid(&self) -> Option<&str> {
		match self.map.get("pid") {
			Some(num) if num.len() == 9 => return Some(num),
			_ => return None
		}
	}
}


fn split_field(data: &str) -> (&str, &str) {
	let parts: Vec<&str> = data.split(':').collect();
	return  (parts[0], parts[1])
}

fn scan_passport(data: &str) -> HashMap<&str, &str> {
	let fields: Vec<(&str, &str)> = data.split_whitespace().map(|field| split_field(field)).collect();
	let mut field_map = HashMap::new();

	for field in fields {
		field_map.insert(field.0, field.1);
	}
	return field_map;
}

fn check_passport(data: &str) -> bool {
	let field_map = scan_passport(data);
	return check_passport_from_hash(&field_map);
}

fn check_passport_from_hash(data: &HashMap<&str, &str>) -> bool {
	let expected_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	for field in &expected_fields {
		if !data.contains_key(field) {
			return false
		}
	}

	return true
}

pub fn part1(data: &String) -> i32 {
	let potential_passports: Vec<&str> = data.split("\r\n\r\n").collect();

	return potential_passports.iter().filter(|potential| check_passport(potential)).count() as i32;
}

fn validate_passport(passport: &Passport) -> bool {
	if passport.byr().is_none() { return false; };
	if passport.iyr().is_none() { return false; };
	if passport.eyr().is_none() { return false; };
	if passport.hgt().is_none() { return false; };
	if passport.hcl().is_none() { return false; };
	if passport.ecl().is_none() { return false; };
	if passport.pid().is_none() { return false; };
	
	return true;
}

pub fn part2(data: &String) -> i32 {
	let potential_passports = data.split("\r\n\r\n").map(|data| scan_passport(&data));
	let passports: Vec<Passport> = potential_passports.map(|p| Passport { map: p }).collect();

	return passports.iter().filter(|passport| validate_passport(passport)).count() as i32;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parsing() {
		let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

		let potential_passports: Vec<&str> = data.split("\n\n").collect();
		let count = potential_passports.iter().filter(|potential| check_passport(potential)).count();

		assert_eq!(count, 2);
	}
}