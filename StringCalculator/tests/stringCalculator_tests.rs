use std::str::FromStr;

fn compute(input: &str) -> isize {
	if is_empty(input) {
		0
	}
	else {
		sum_of(extract_numbers(input))
	}
}

fn is_empty (input: &str) -> bool {
	input.len() == 0
}

fn extract_numbers(input: &str) -> Vec<isize> {
	input.split_str(",").map(|x| parse_int(x)).collect()
}

fn parse_int(input: &str) -> isize {
	FromStr::from_str(input).unwrap()
}

fn sum_of(numbers: Vec<isize>) -> isize {
	let mut sum = 0is;
	for &s in numbers.iter() {
		sum = sum + s
	}
	sum
}


#[test]
fn empty_got_zero() {
	assert!(compute("") == 0);
}

#[test]
fn one_got_one() {
	assert!(compute("1") == 1);
}

#[test]
fn eighteen_got_eighteen() {
	assert!(compute("18") == 18);
}

#[test]
fn one_plus_two_got_three() {
	assert!(compute("1,2") == 3);
}
