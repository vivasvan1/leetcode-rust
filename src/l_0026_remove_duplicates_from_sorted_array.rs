pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut nums = nums;
        if nums.len() == 0 {
            return 0;
        }

        if nums.len() == 1 {
            return 1;
        }

        let mut i = 0;
        let mut j = 1;
        while j != nums.len() {
            // println!("i : {}, j : {}, nums[i] : {}, nums[j] : {}", i, j, nums[i], nums[j]);
            if nums[i] == nums[j] {
                j += 1;
            } else {
                i += 1;
                nums[i] = nums[j];
            }
        }
        i as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    }
}
