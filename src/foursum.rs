use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort();
        // println!("{:?}",nums);
        let mut hashmap: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            hashmap.insert(*num, index);
        }
        let mut res: Vec<Vec<i32>> = vec![];

        let mut i = 0;
        while i < nums.len() - 3 {
            // for i in 0..nums.len() - 3 {
            let mut j = i + 1;
            while j < nums.len() - 2 {
                let mut k = j + 1;
                while k < nums.len() - 1 {
                    let l_needed = target - (nums[i] + nums[j] + nums[k]);

                    if hashmap.contains_key(&l_needed) && hashmap[&l_needed] > k + 1 {
                        res.push(vec![nums[i], nums[j], nums[k], l_needed]);
                    }
                    k = hashmap[&nums[k]] + 1_usize;
                }
                j = hashmap[&nums[j]] + 1_usize;
            }
            i = hashmap[&nums[i]] + 1_usize;
        }
        // println!("{:?}", res);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        )
    }
}
