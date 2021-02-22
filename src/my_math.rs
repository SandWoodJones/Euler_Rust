pub fn trial_division(mut n: u64) -> Vec<u64> { // returns vector of all prime factors of 'n', if its length is 2, 'n' is prime
	if n <= 1 {
		return vec![n];
	}

	let mut v = Vec::new();
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
	if n != 1 { v.push(n); }
	v
}

pub fn is_prime(n: u64) -> bool {
	if n <= 1 { return false; } // 0 & 1 aren't prime
	if n < 4 { return true; } // 2 & 3 are prime
	if n % 2 == 0 { return false; } // 2 is the only even prime number
	if n < 9 { return true; } // every odd single digit number except 9 is prime

	let root = (n as f64).sqrt().floor() as u64;
	let mut factor = 5; // 5 is the smallest number that can be written as 6k+/-1 (6*1-1).
	while factor <= root { // any number N can only have 1 factor greater than its square root, and that is N itself.
		if n % factor == 0 { return false; } // this is equivalent to 6k - 1
		if n % factor + 2 == 0 { return false; } // and this is equivalent to 6k + 1
		factor += 6; // every prime greater than 3 can be written as 6k+/-1. 7 = 6*1+1; 11 = 6*2-1; 101 = 6*17-1
	}

	true
}

pub fn generate_primes(limit: u64) -> Vec<u64> { // gets all prime numbers less or equal to limit
	let mut v = vec![2, 3];
	let mut i = 5;
	while i <= limit {
		if is_prime(i) { v.push(i); }
		if is_prime(i + 2) { v.push(i + 2) }
		i += 6;
	}
	v
}

pub fn get_prime(n: u64) -> u64 { // gets the Nth prime number
	if n <= 2 {
		return n+1; // the first prime is 2 and the second is 3
	}

	let mut found = 2; // we already got 2 and 3
	let mut i = 5;

	loop {
		if is_prime(i) {
			found += 1;
			if found == n { return i; }
		}
		if is_prime(i + 2) {
			found += 1;
			if found == n { return i + 2; }
		}
		i += 6; // every prime greater than 3 can be represented as 6k+/-1
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

pub fn sum_of_squares(n: i32) -> i32 {
	let mut sum = 0;
	for i in 1 ..= n {
		sum += i.pow(2);
	}
	sum
}

pub fn square_of_sum(n: i32) -> i32 {
	let mut sum = 0;
	for i in 1 ..= n {
		sum += i;
	}
	sum.pow(2)
}

pub fn generate_py_triplet(n: i32, m: i32) -> Option<[i32; 3]> { // returns a pythagorean triple, from 2 numbers
	if m <= n || n <= 0 { return None }
	let a = m.pow(2) - n.pow(2);
	let b = 2 * m * n;
	let c = m.pow(2) + n.pow(2);

	Some([a, b, c])
}

pub fn get_all_divisors(n: u64) -> Vec<u64> { // returns an unordered list of all numbers that divide n evenly
	let mut divisors = Vec::new();

	let root = (n as f64).sqrt().round() as u64;
	for i in 1 ..= root {
		if n % i == 0 {
			divisors.push(i);
			if n / i != i {
				divisors.push(n / i);
			}
		}
	}
	divisors
}