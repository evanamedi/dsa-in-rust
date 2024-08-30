// to run locally and submit

use std::collections::HashMap;

pub struct Solution;

impl Solution {
	pub fn length_of_longest_substring(s: String) -> i32 {
		let mut char_map = HashMap::new();
		let mut max_len = 0;
		let mut start = 0;

		for (i, c) in s.chars().enumerate() {
			if let Some(&index) = char_map.get(&c) {
				start = start.max(index + 1);
			}
			char_map.insert(c, i);
			max_len = max_len.max(i - start + 1);
		}

		max_len as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_length_of_longest_substring() {
		assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring("abcdef".to_string()), 6);
	}
}
