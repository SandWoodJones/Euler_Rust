/* 	2520 is the smallest number that can be divided by each of the numbers
	from 1 to 10 without any remainder. What is the smallest positive number
	that is evenly divisible by all of the numbers from 1 to 20? */


#[allow(dead_code)]
pub fn answer() -> i32 {
	let mut result = 20;
	loop {
		if evenly_divisible_by_20(result) {
			break;
		} else {
			result += 2;
		}
	}
	result
}

fn evenly_divisible_by_20(n: i32) -> bool {
	if n % 2 != 0 { return false; }

	for i in 3 ..= 20 {
		if n % i != 0 {
			return false;
		}
	}

	true
}