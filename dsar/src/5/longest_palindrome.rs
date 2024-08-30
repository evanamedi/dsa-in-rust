pub struct Solution

impl Solution {
	pub fn longest_palindrome(s: String) -> String {
		let s_chars: Vec<char> = s.chars().collect();
		let len = s.len();
		let (mut start, mut end) = (0, 0);

		for i in 0..len {
			let len1 = Solution::expand_around_center(&s_chars, i, i);
			let len2 = Solution::expand_around_center(&s_chars, i, i + 1);
			let max_len = len1.max(len2);

			if max_len > (end - start) {
				start = i.saturating_sub((max_len - 1) / 2);
				end = i + max_len / 2;
			}
		}

		s_chars[start..=end].iter().collect()
	}

	fn expand_around_center(s: &[char], left: usize, right: usize) -> usize {
		let (mut l, mut r) = (left as isize, right as isize);
		while l >= 0 && r < s.len() as isize && s[l as usize] == s[r as usize] {
			l -= 1;
			r += 1;
		}

		(r - l - 1) as usize
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
        assert_eq!(Solution::longest_palindrome("racecar".to_string()), "racecar".to_string());
    }
}