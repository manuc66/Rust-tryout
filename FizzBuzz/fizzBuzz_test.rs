fn fizz_buzz(i: int) -> String {
	if ((i % 5) == 0) && ((i % 3) == 0) {
		"FizzBuzz".to_string()
	}
	else if (i % 5) == 0 {
		"Buzz".to_string()
	}
	else if (i % 3) == 0 {
		"Fizz".to_string()
	}
	else {
		i.to_string()
	}
}


#[test]
fn one_got_one() {
	assert!(fizz_buzz(1) == "1".to_string());
}
#[test]
fn three_got_fizz() {
	assert!(fizz_buzz(3) == "Fizz".to_string());
}
#[test]
fn five_got_buzz() {
	assert!(fizz_buzz(5) == "Buzz".to_string());
}
#[test]
fn fifteen_got_fizzbuzz() {
	assert!(fizz_buzz(15) == "FizzBuzz".to_string());
}