//! # In range
//!
//! Returns true if a number is in range.
//!
//! ## Usage
//!
//! First, install the library with
//!
//! ```bash
//! cargo add in_range
//! ```
//!
//! Then, in code:
//!
//! ```
//! use in_range::in_range;
//!
//! // Args: value, min, max
//! let result = in_range(10, 1, 15);
//!
//! print!("{}", result);
//! // Prints true;
//! ```
//!
//! You can also use `not_in_range` to return *false* when value is in range.
//!
//! ```
//! use in_range::not_in_range;
//!
//! // Args: value, min, max
//! let result = not_in_range(25, 1, 15);
//!
//! print!("{}", result);
//! // Prints true;
//! ```

/// Return true if an i32 value is within range.
pub fn in_range(value: i32, min: i32, max: i32) -> bool {
	value >= min && value <= max
}

/// Return false if an i32 value is within range.
pub fn not_in_range(value: i32, min: i32, max: i32) -> bool {
	!in_range(value, min, max)
}

/// Return true if a u32 value is within range.
pub fn in_range_u32(value: u32, min: u32, max: u32) -> bool {
	value >= min && value <= max
}

/// Return false if a u32 value is within range.
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
			panic!("u32 number in range test failed. Number not in range.")
		}
	}

	#[test]
	fn not_in_range_u32_test() {
		let result = not_in_range_u32(100, 1, 15);
		if !result {
			panic!("u32 number not in range test failed. Number is in range.")
		}
	}
}
