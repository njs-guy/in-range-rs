pub fn in_range(value: i32, min: i32, max: i32) -> bool {
	value >= min && value <= max
}

pub fn not_in_range(value: i32, min: i32, max: i32) -> bool {
	!in_range(value, min, max)
}

pub fn in_range_u32(value: u32, min: u32, max: u32) -> bool {
	value >= min && value <= max
}

pub fn not_in_range_u32(value: u32, min: u32, max: u32) -> bool {
	!in_range_u32(value, min, max)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn in_range_test() {
		let result = in_range(10, 1, 15);
		if !result {
			panic!("Number in range test failed. Number not in range.")
		}
	}

	#[test]
	fn not_in_range_test() {
		let result = not_in_range(100, 1, 15);
		if !result {
			panic!("Number not in range test failed. Number is in range.")
		}
	}

	#[test]
	fn in_range_u32_test() {
		let result = in_range_u32(10, 1, 15);
		if !result {
			panic!("Number in range test failed. Number not in range.")
		}
	}

	#[test]
	fn not_in_range_u32_test() {
		let result = not_in_range_u32(100, 1, 15);
		if !result {
			panic!("Number not in range test failed. Number is in range.")
		}
	}
}
