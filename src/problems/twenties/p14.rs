/*	The following iterative sequence is defined for the set of positive integers:
		n -> n/2 (if n is even)
		n -> 3n + 1 (if n is odd)
	Using the rule above and starting with 13, we generate the following sequence:
		13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
	It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
	Although it has not been proved yet (Collatz Problem), it is thought that all
	starting numbers finish at 1.
	Which starting number, under one million, produces the longest chain?
	NOTE: Once the chain starts the terms are allowed to go above one million. */

pub fn answer() -> u64 {
	let mut largest = (0, 0); // (start_number, length)

	for i in 1 ..= 1_000_000 {
		let seq = collatz(i).unwrap();
		if seq.len() > largest.1 {
			largest.0 = i;
			largest.1 = seq.len()
		}
	}
	largest.0
}

fn collatz(start: u64) -> Option<Vec<u64>> { // returns a collatz sequence from start to 1
	if start == 0 { return None; }
	if start == 1 { return Some(vec![start]); }
	let mut result = Vec::new();

	let mut start = start;
	while start != 1 {
		result.push(start);
		if start % 2 == 0 {
			start /= 2;
		} else {
			start = (start * 3) + 1
		}
	}

	result.push(1);
	Some(result)
}