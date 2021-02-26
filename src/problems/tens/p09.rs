/* 	A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
	a^2 + b^2 = c^2. For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2. There exists exactly
	one Pythagorean triplet for which a + b + c = 1000. Find the product abc. */

use crate::my_math::generate_py_triplet;

#[allow(dead_code)]
pub fn answer() -> i32 {
	for i in 1 ..= 1000 {
		for j in i + 1 ..= 1000 {
			let triplet = generate_py_triplet(i, j).expect("Failed to generate pythagorean triplet");
			if triplet[0] + triplet[1] + triplet[2] == 1000 {
				return triplet[0] * triplet[1] * triplet[2];
			}
		}
	}
	0
}