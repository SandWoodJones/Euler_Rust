mod problems;
pub mod my_math;

use problems::twenties::p14;

fn main() {
	let answer = p14::answer();

	println!("{}", answer);
}