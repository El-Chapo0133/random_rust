use std::time::SystemTime;


mod data {
	// max i64 value
	pub const MAX: i128 = 922337203685775807;
	// imperative choosen value
	pub const MULTIPLICATOR: i64 = 16807;
}

struct Random {
	seed: i64, // seed used to generate the random, based on unix time and imperatives values
}
impl Random {
	/*pub*/ fn new() -> Random {
		Random {
			seed: (get_time_unix() * data::MULTIPLICATOR + 1) % (power_of(2,31) - 1),
		}
	}
	// use this function to get the next random
	/*pub*/ fn next(&mut self, min: i64, max: i64) -> i64 {
		round_f_to_i((min + (max - min)) as f64 * self.pseudo_uniform())
	}
	fn pseudo_uniform(&mut self) -> f64 {
		let modulo: i64 = power_of(2,31) - 1;
		self.seed = (self.seed * data::MULTIPLICATOR + 1) % modulo; // re-set the seed
		return self.seed as f64 / modulo as f64;
	}
}

fn main() {
	let mut random: Random = Random::new();
	println!("{}", random.next(0, 1));

	
	// generate 100 randoms
	for _ in 0..100 {
		println!("{:.0}", random.next(0, 1));
	}
	
}

/// get the power of a number from the exposant
/// panic! if the number is too big
/// shouldn't append, cuz i only make 2^31
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
	let mut index: i64 = 0; // which index do we are
	loop {
		index += 1;
		let next_dec: i64 = get_dec(&input, index); // get the dec at the index given
		if next_dec != 5 {
			if next_dec < 5 {
				return input as i64;
			} else {
				return input as i64 + 1;
			}
		} // if next_dec == 5, make another turn
	}
}
// get the index decimal
fn get_dec(input: &f64, index: i64) -> i64 {
	(input * (power_of(10, index)) as f64) as i64 % 10
}

// get the unix time
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
		let inputs: Vec<i64> = [1,2,3,4,5].to_vec();
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
	fn test_get_dec() {
		let mut inputs: Vec<f64> = [1.1, 1.2, 0.4, 1245.7].to_vec();
		let mut outputs: Vec<i64> = [1,2,4,7].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(get_dec(&inputs[i], 1), outputs[i]);
		}
		inputs = [1.12, 1.25, 0.41, 1245.447].to_vec();
		outputs = [2,5,1,4].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(get_dec(&inputs[i], 2), outputs[i]);
		}
		inputs = [1.125, 15.25235, 0.4112, 1245.4347].to_vec();
		outputs = [5,2,1,4].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(get_dec(&inputs[i], 3), outputs[i]);
		}
	}
	#[test]
	fn test_round_f_to_i() {
		let inputs: Vec<f64> = [125.6, 12.73, 123.556, 0.555551, 0.555559].to_vec();
		let outputs: Vec<i64> = [126,13,124,0,1].to_vec();
		for i in 0..inputs.len() {
			assert_eq!(round_f_to_i(inputs[i]), outputs[i]);
		}
	}
}