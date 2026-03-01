#[cfg(test)]
#[test]

fn so () {
	use super::*;
	assert_eq!(isNonTrivialRotation("abcd", "abcd"), false);
	assert_eq!(isNonTrivialRotation("abcd", "bcda"), true);
	assert_eq!(isNonTrivialRotation("abcd", "abce"), false);
}
#[test]
fn rt () {
	use super::*;
	let data2 = [
		("abcde", "cdeab", true),
		("abcde", "cdeab",true),
		("abcde", "bcdea",true),
		("abcde", "eabcd",true),
		("abcde", "deabc",true),
		("hello", "llohe",true),
		("abc", "abc", false),
		("hello", "hello", false),
		("aa", "aa", false),
		("aaa", "aaa", false),
		("a", "b", false),
		("abc", "xyz", false),
		("abc", "acb", false),
		("abc", "bac", false),
	];

	for d in data2.iter() {
		println!("{}\n{}\n{}", d.0, d.1, d.2);
		assert_eq!(isNonTrivialRotation(d.0, d.1), d.2);
	}
}
