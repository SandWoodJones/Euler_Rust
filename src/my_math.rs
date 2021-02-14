pub fn trial_division(mut n: u64) -> Vec<u64> { // returns vector of all prime factors of 'n', if its length is 2, 'n' is prime
	let mut v = vec![1];
	while n % 2 == 0 { // divide by 2 until we can't anymore
		v.push(2);
		n /= 2;
	}

	let mut factor = 3; // after we're done with 2 we go on to 3
	while factor * factor <= n {
		if n % factor == 0 {
			v.push(factor);
			n /= factor;
		} else {
			factor += 2; // by adding 2 here we're only using odd factors, skipping over multiples of 2
		}
	}
	v.push(n);
	v
}

pub fn is_prime(n: u64) -> bool {
	match trial_division(n).len() {
		2 => true,
		_ => false
	}
}

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