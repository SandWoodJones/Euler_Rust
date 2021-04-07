use std::io;

// returns vector of all prime factors of 'n', if its length is 2, 'n' is prime
pub fn trial_division(mut n: u64) -> Vec<u64> {
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

	// any number N can only have 1 factor greater than its square root, and that is N itself.
	while factor <= root {
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
		// isolates the last digit of 'n', 10 is used since we're working with base-10 numbers
		let last_digit = n % 10; 
        
		// by multiplying by 10 we make the 'ones' column the 'tens' column and so on
		reverse = (reverse * 10) + last_digit; 
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

// returns a pythagorean triple, from 2 numbers
pub fn generate_py_triplet(n: i32, m: i32) -> Result<[i32; 3], String> { 
	if n <= 0 { return Err(String::from("Error: n value must be greater than 0.")); }
	if m <= n { return Err(String::from("Error: m value must be greater than n value.")); }

	let a = m.pow(2) - n.pow(2);
	let b = 2 * m * n;
	let c = m.pow(2) + n.pow(2);

	Ok([a, b, c])
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

// returns a vector of a collatz sequence from start to 1
pub fn collatz(start: u64) -> Result<Vec<u64>, io::Error> {
	if start == 0 {
		return Err(io::Error::new(io::ErrorKind::InvalidInput, "0 is an invalid starting point"));
	}

	if start == 1 { return Ok(vec![start]); }
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
	Ok(result)
}

pub fn factorial(n: u32) -> u32 {
	let mut result = 1;
	if n <= 1 { return result; }
	for i in 1 ..= n {
		result *= i;
	}
	result
}

pub fn bin_coeff(n: u64, k: u64) -> u64 {
	use std::cmp::min;

	// `bin_coeff(n, k)` is the same as `bin_coeff(n, n-k)` so we take the smaller of the two:
	let new_k = min(k, n - k);

	let mut result = 1;
	for i in 1 ..= new_k {
		result *= n - new_k + i;
		result /= i;
	}

	result
}
