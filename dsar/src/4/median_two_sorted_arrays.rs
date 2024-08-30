// run locally and submit

pub struct Solution;

impl Solution {
	pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
		let (mut nums1, mut nums2) = if nums1.len() > nums2.len() {
			(nums2, nums1)
		} else {
			(nums1, nums2)
		};

		let m = nums1.len();
		let n = nums2.len();
		let mut imin = 0;
		let mut imax = m;
		let half_len = (m + n + 1) / 2;

		while imin <= imax {
			let i = (imin + imax) / 2;
			let j = half_len - i;

			if i < m && nums1[i] < nums2[j - 1] {
				imin = i + 1;
			} else if i > 0 && nums1[i - 1] > nums2[j] {
				imax = i - 1;
			} else {
				let max_of_left = if i == 0 {
					nums2[ j - 1]
				} else if j == 0 {
					nums1[i - 1]
				} else {
					nums1[i - 1].max(nums2[j - 1])
				};

				if (m + n) % 2 == 1 {
					return max_of_left as f64;
				}

				let min_of_right = if i == m {
					nums2[j]
				} else if j == n {
					nums1[i]
				} else {
					nums1[i].min(nums2[j])
				};

				return (max_of_left as f64 + min_of_right as f64) / 2.0;
			}
		}

		0.0
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    }
}
