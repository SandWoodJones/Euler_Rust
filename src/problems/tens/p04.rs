/*	A palindromic number reads the same both ways. The largest palindrome made from
	the product of two 2-digit numbers is 9009 = 91 × 99. Find the largest
	palindrome made from the product of two 3-digit numbers. */

use crate::my_math::is_palindrome;

#[allow(dead_code)]
pub fn answer() -> i32 {
	let mut largest = 0;
	for i in 100 ..= 999 {
		/*	to avoid checking a number more than once (E.g '696969' is checked when {i=132, j=528} & {i=528, j=132}),
			we can assume i <= j */
		for j in i ..= 999 {
			let r = i * j;
			if r > largest {
				if is_palindrome(r) {
					largest = r;
				} else {
					continue;
				}
			}
		}
	}
	largest
}