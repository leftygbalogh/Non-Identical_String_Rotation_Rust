#[cfg(test)]
#[test]

fn so () {
	use super::*;

	assert_eq!(isNonTrivialRotation("abcd", "abcd"), true);
	assert_eq!(isNonTrivialRotation("abcd", "bcda"), true);
	assert_eq!(isNonTrivialRotation("abcd", "abce"), false);
}
#[test]
fn rt () {
	use super::*;
	let data = [("asd", "asd", true)];

	for d in data.iter() {
		assert_eq!(isNonTrivialRotation(d.0, d.1), d.2);
	}
}
