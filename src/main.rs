mod problem1;
mod problem2;

pub mod my_math;

fn main() {
	let answer1 = problem1::answer();
	let answer2 = problem2::answer();

	println!("p1: {}", answer1);
	println!("p2: {}", answer2);
}