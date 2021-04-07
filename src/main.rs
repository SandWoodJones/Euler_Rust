mod problems;
pub mod my_math;

use problems::twenties::p15;

fn main() {
	let answer = p15::answer();

	println!("{}", answer);
}
