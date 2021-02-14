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

pub fn length_of_number(mut n: i32) -> usize {
	if n == 0 { return 1; }
	let mut i = 0;

	while n > 0 {
		n /= 10; // by dividing by 10 each time, we effectively remove one of the columns of the 'n' number
		i += 1;
	}

	i
}

pub fn is_palindrome(mut n: i32) -> bool {
	if length_of_number(n) == 1 { return true; } // single-digit numbers are always palindromes.

	let start = n;
	let mut reverse = 0;
	while n > 0 {
		let last_digit = n % 10; // isolates the last digit of 'n', 10 is used since we're working with base-10 numbers
		reverse = (reverse * 10) + last_digit; // by multiplying by 10 we make the 'ones' column the 'tens' column and so on
		n /= 10; // now we remove the last digit of 'n' and continue the loop
	}

	if reverse == start {
		return true;
	}
	false
}