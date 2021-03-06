/* 	By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
	the 6th prime is 13. What is the 10 001st prime number? */

use crate::my_math::get_prime;

#[allow(dead_code)]
pub fn answer() -> u64 {
	get_prime(10001)
}