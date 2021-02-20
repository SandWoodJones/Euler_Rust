/*	The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17. Find the sum of all the primes
	below two million. */

use crate::my_math::generate_primes;

#[allow(dead_code)]
pub fn answer() -> u64 {
	let primes = generate_primes(2000000);
	primes.iter().sum()
}