mod problems;
pub mod my_math;

use problems::twenties::p11;

fn main() {
	let answer = p11::answer();

	println!("{}", answer);
}