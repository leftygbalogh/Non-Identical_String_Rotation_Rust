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
	let vecDeq1 = std::collections::VecDeque::from(vec![s1.chars()]);
	let vecDeq2 = std::collections::VecDeque::from(vec![s2.chars()]);
	println!("{:?} vs a{:?}", vecDeq1, vecDeq1);
	//println!("{:?}", vecDeq1.iter().eq(&'a'));
	true

}



fn main() {
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();

	let s1 = stdin_iterator.next().unwrap().unwrap();

	let s2 = stdin_iterator.next().unwrap().unwrap();

	let result = isNonTrivialRotation(&s1, &s2);

	println!("{}", if result { 1 } else { 0 });
}
