pub fn reverse_string(s: String) -> String {
	return s.chars().rev().collect();
}

/// Returns:
///
/// `-1` if a is less than b
///
/// `0` if a is equal than b
///
/// `1` if a is bigger than b
///
pub fn compare_str_number(a: &String, b: &String) -> i8 {
	if a.len() < b.len() { return -1; }
	if a.len() > b.len() { return 1; }

	// Then they are equal on length
	let mut index: usize = 0;
	while index < a.len() {
		if (a.chars().nth(index).unwrap() as u8) < (b.chars().nth(index).unwrap() as u8) { return -1; }
		if (a.chars().nth(index).unwrap() as u8) > (b.chars().nth(index).unwrap() as u8) { return 1; }

		index += 1;
	}

	return 0;
}
