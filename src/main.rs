use std::time::SystemTime;


mod data {
	pub const MAX: i128 = 922337203685775807;
	pub const MULTIPLICATOR: i64 = 16807;

}

struct Random {
	buffer: i64,
}
impl Random {
	fn new() -> Random {
		Random {
			buffer: (get_time_unix() * data::MULTIPLICATOR + 1) % (power_of(2,31) - 1),
		}
	}
	fn next(&mut self, min: i64, max: i64) -> i64 {
		round_f_to_i((min + (max - min)) as f64 * self.pseudo_uniform())
	}
	fn pseudo_uniform(&mut self) -> f64 {
		let modulo: i64 = power_of(2,31) - 1;
		self.buffer = (self.buffer * data::MULTIPLICATOR + 1) % modulo;
		return self.buffer as f64 / modulo as f64;
	}
}
struct Vector {
	data: [i64],
}
impl Vector {
	fn new() -> Vector {
		Vector {
			data: [],
		}
	}
	fn push(&mut self, input: i64) {
		
	}
}

fn main() {
	let mut random: Random = Random::new();
	println!("{:.0}", random.next(0, 1));
}


fn power_of(base: i64, exposant: i64) -> i64 {
	let mut to_return: i128 = base as i128;
	for _ in 1..exposant {
		to_return *= base as i128;
	}
	if to_return > data::MAX {
		panic!(format!("{0}^{1} is too big for an i64", base, exposant));
	}
	return to_return as i64;
}
fn round_f_to_i(input: f64) -> i64 {
	let stringed = format!("{:.2}", input);
	let splitted = split_once(stringed);
}
fn split_once(input: String) -> [&str;2] {
	let char_to_find = ".";
	let to_return = [""; 2];
	for 
}
fn to_cells(input: String) -> 
fn length(input: String) -> i64 {

}
fn to_string(input: i64) -> Option<&'static str> {
	let chars = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "."];
	let ints = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	for i in 0..11 {
		if ints[i] == input {
			return Some(chars[i]);
		}
	}
	None
}
fn to_int(input: &str) -> Option<i64> {
	let chars = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "."];
	let ints = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	for i in 0..11 {
		if chars[i] == input {
			return Some(ints[i]);
		}
	}
	None
}


fn get_time_unix() -> i64 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => return n.as_secs() as i64,
		Err(why) => {
			println!("Can't get unix time, {}", why);
			std::process::exit(1)
		},
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_power_of() {
		let mut inputs: Vec<i64> = [1,2,3,4,5].to_vec();
		let mut outputs: Vec<i64> = [1,4,9,16,25].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(power_of(inputs[i],2), outputs[i]);
		}
		outputs = [1,8,27,64,125].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(power_of(inputs[i],3), outputs[i]);
		}
	}
	#[test]
	fn test_to_string() {
		let inputs: Vec<i64> = [1,2,3,4,5,11].to_vec();
		let outputs: Vec<Option<&'static str>> = [Some("1"),Some("2"),Some("3"),Some("4"),Some("5"),None].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(to_string(inputs[i]), outputs[i]);
		}
	}
	#[test]
	fn test_to_int() {
		let inputs: Vec<&str> = ["1","2","3","4","5",".","10"].to_vec();
		let outputs: Vec<Option<i64>> = [Some(1),Some(2),Some(3),Some(4),Some(5),Some(10),None].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(to_int(inputs[i]), outputs[i]);
		}
	}
}