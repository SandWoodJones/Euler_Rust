mod problems;
pub mod my_math;

use problems::tens::p10;

fn main() {
	let answer = p10::answer();

	println!("{}", answer);
}