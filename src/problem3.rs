/*	The prime factors of 13195 are 5, 7, 13 and 29. What is the largest prime factor
	of the number 600851475143 ? */

use crate::my_math::trial_division;
use std::cmp::max;

pub fn answer() -> u64 {
	let n: u64 = 600851475143;
	let mut largest = 1;
	for i in trial_division(n) {
		largest = max(i, largest);
	}
	largest
}