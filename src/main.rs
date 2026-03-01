mod tests;


use std::io::{self, BufRead};



/*
 * Complete the 'isNonTrivialRotation' function below.
 *
 * The function is expected to return a BOOLEAN.
 * The function accepts following parameters:
 *  1. STRING s1
 *  2. STRING s2
 */

fn isNonTrivialRotation(s1: &str, s2: &str) -> bool {
	use std::collections::VecDeque;

	if s1.eq_ignore_ascii_case(s2) {
		return false;
	}

	let mut vecDeq1 = std::collections::VecDeque::new();
	for x in s1.chars() {
		vecDeq1.push_back(x);
	};

	let mut vecDeq2 = std::collections::VecDeque::new();
	for x in s2.chars() {
		vecDeq2.push_back(x);
	};



	// println!("{:?} ", vecDeq1);
	// println!("{:?} ", vecDeq2);
	let mut index = 0;
	let mut vecDeq3:VecDeque<char> = std::collections::VecDeque::new();

	while index < vecDeq1.len() {

		let mut c = vecDeq2.pop_front();
		//println!("index is: {} what is popped: {:?}", index, &c);
		if &c != &Option::from(vecDeq1[index]) {
			// println!("character {:?} ", &c);
			// println!("VD2 before: {:?} ", &vecDeq2);
			vecDeq3.push_back(c.unwrap().to_ascii_lowercase());
//			println!("VD3 intermediate: {:?} ", &vecDeq3);
			vecDeq2.append(&mut vecDeq3);
//			println!("VD2 intermediate: {:?} ", &vecDeq2);
			if &vecDeq1.iter().collect::<String>() == &vecDeq2.iter().collect::<String>()
			{
				return true;
			}
		}
		else {
			vecDeq3.push_back(c.unwrap().to_ascii_lowercase());

		}
		index += 1;

	}
//	println!("D1: {:?}", vecDeq1);
//	println!("D2: {:?}", vecDeq2);
	vecDeq1.iter().collect::<String>() == vecDeq2.iter().collect::<String>()

}



fn main() {
	println!("\x1b[2J\x1b[H\x1b[3J");
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let s1 = stdin_iterator.next().unwrap().unwrap();

	let s2 = stdin_iterator.next().unwrap().unwrap();

	let result = isNonTrivialRotation(&s1, &s2);

	println!("{}", if result { 1 } else { 0 });
}
