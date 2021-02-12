pub fn fibonacci(n: i32) -> i32 {
	if n <= 1 { return n; }

	let mut tmp;
	let mut p_first = 1;
	let mut p_second = 1;

	for _ in 2 .. n {
		tmp = p_second + p_first;
		p_first = p_second;
		p_second = tmp;
	}
	p_second
}