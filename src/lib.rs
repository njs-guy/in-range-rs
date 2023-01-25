pub fn in_range(value: i32, min: i32, max: i32) -> bool {
	let mut result = false;

	if value >= min && value <= max {
		result = true;
	}

	result
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
}
