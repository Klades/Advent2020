pub fn part1(data: &Vec<String>) -> i32 {
	return *get_all_ids(data).iter().max().unwrap();
}

fn get_all_ids(data: &Vec<String>) -> Vec<i32> {
	let seats = Seat::build_seats(&data);
	return seats.iter().map(|seat| seat.id).collect();
}

pub fn part2(data: &Vec<String>) -> i32 {
	let seats = Seat::build_seats(&data);
	let seat_ids = get_all_ids(data);

	let mut plane = Plane::new(data.len());

	for seat in seats {
		let row = seat.row;
		
		plane.rows[row].seats.push(seat);
	}

	let empty_seats = plane.find_empty_seats();
	let empty_ids: Vec<i32> = empty_seats.iter().map(|seat| seat.id).collect();

	for id in empty_ids {
		let plus_one = seat_ids.iter().find(|&&x| x == id + 1);
		let minus_one = seat_ids.iter().find(|&&x| x == id - 1);

		if plus_one.is_some() && minus_one.is_some() {
			return id;
		}
	}

	return -1;
}


#[derive(Clone)]
struct Seat {
	id: i32,
	row: usize,
	col: usize
}

#[derive(Clone)]
struct Row {
	seats: Vec<Seat>,
}

impl Row {
	fn new() -> Row {
		let row = Row { seats: Vec::new() };

		return row;
	}

	fn find_empty_seat(&self) -> Option<Seat> {
		let cols: Vec<usize> = self.seats.iter().map(|seat| seat.col).collect();
		let row = self.seats[0].row;

		for col in 0..7 {
			match cols.iter().find(|&&x| x == col) {
				Some(_) => continue,
				None => return Some(Seat { row: row, col: col, id: seat_id(row as i32, col as i32) })
			}
		}

		return None;
	}
}

struct Plane {
	rows: Vec<Row>,
}

impl Plane {
	fn new(size: usize) -> Plane {
		let mut plane = Plane { rows: Vec::new() };
		plane.rows.resize(size, Row::new());

		return plane;
	}

	fn find_empty_seats(&self) -> Vec<Seat> {
		let mut empty_seats = Vec::<Seat>::new();

		for row in &self.rows {
			if row.seats.len() == 0 || row.seats.len() == 8 { continue; }

			match row.find_empty_seat() {
				Some(s) => empty_seats.push(s),
				_ => continue
			}
		}

		return empty_seats;
	}
}

impl Seat {
	fn build_seats(data: &Vec<String>) -> Vec<Seat> {
		return data.iter().map(|line| Seat::parse(line.as_bytes()).unwrap()).collect();
	}

	fn parse(code: &[u8]) -> Option<Seat> {
		let row = {
			match decode_row(&code) {
				Ok(value) => value,
				_ => return None
			}
		};

		let col = {
			match decode_col(&code) {
				Ok(value) => value,
				_ => return None
			}
		};

		return Some(Seat { id: seat_id(row, col), row: row as usize, col: col as usize});
	}
}

fn decode_seat(code: &[u8], lower_code: u8, upper_code: u8, max_seat: i32) -> Result<i32, &str> {
	let mut seat_min = 0;
	let mut seat_max = max_seat;
	
	for dir in code {
		if seat_min + 1 == seat_max {
			match dir {
				x if *x == lower_code => return Ok(seat_min),
				x if *x == upper_code => return Ok(seat_max),
				_ => return Err("unexpected character")
			}
		}

		else {
			match dir {
				x if *x == lower_code => seat_max = (seat_min + seat_max) / 2,
				x if *x == upper_code => seat_min = (seat_min + seat_max + 1) / 2,
				_ => return Err("unexpected character")
			}
		}
	}

	if seat_min != seat_max { return Err("didn't narrow down to one seat"); }

	return Ok(seat_max);
}

fn seat_id(row: i32, col: i32) -> i32 {
	return row * 8 + col;
}

fn decode_row(code: &[u8]) -> Result<i32, &str> {
	return decode_seat(&code, 'F' as u8, 'B' as u8, 127);
}

fn decode_col(code: &[u8]) -> Result<i32, &str> {
	return decode_seat(&code[7..], 'L' as u8, 'R' as u8, 7);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_works() {
		let code = b"FBFBBFFRLR";

		assert_eq!(decode_row(&code[..]).unwrap(), 44);
		assert_eq!(decode_col(&code[..]).unwrap(), 5);

		assert_eq!(seat_id(44, 5), 357);
	}
}