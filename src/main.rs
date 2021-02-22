mod problems;
pub mod my_math;

use problems::twenties::p12;

fn main() {
	let answer = p12::answer();

	println!("{}", answer);
}