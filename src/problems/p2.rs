/*	Each new term in the Fibonacci sequence is generated by adding the previous two terms.
	By starting with 1 and 2, the first 10 terms will be: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
	By considering the terms in the Fibonacci sequence whose values do not exceed four million,
	find the sum of the even-valued terms. */

use crate::my_math::fibonacci;

pub fn answer() -> i32 {
	let limit = 4_000_000;
	let mut sum = 0;

	for i in 0 .. limit {
		/* 	in the fibonacci sequence we can see that, every 3rd number is even:
			1, 1, '2', 3, 5, '8', 13, 21, '34', 55, 89, '144'...
			this allows us to avoid running 'fibonacci()' for numbers we know are even &
			then having to check if the values in 'fib' are divisible by 2 */
		if i % 3 != 0 {
			let fib = fibonacci(i);

			if fib > limit {
				break;
			}

			sum += fib;
		}
	}

	sum
}