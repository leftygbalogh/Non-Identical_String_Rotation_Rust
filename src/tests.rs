#[cfg(test)]
#[test]
fn so () {
	use super::*;

	assert_eq!(isNonTrivialRotation("abcd", "abcd"), true);
	assert_eq!(isNonTrivialRotation("abcd", "bcda"), true);
	assert_eq!(isNonTrivialRotation("abcd", "abce"), false);
}
