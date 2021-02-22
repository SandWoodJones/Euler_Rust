mod problems;
pub mod my_math;

use problems::twenties::p13;

fn main() {
	let answer = p13::answer();

	println!("{}", answer);
}