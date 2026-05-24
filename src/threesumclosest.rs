pub struct Solution;

impl Solution {
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {

        // [-4, -1, 1, 2] target = 1
        
        // (i < n-2) .. (i+1 < j < n-1)

        // i=0 | j=1 | v(i) = -4 | v(j) = -1 | -5 | d(t) = -6 | needed = 6 <-> 2 | nums = -4 -1 2 sum = -3 best_d=4 
        // i=0 | j=2 | v(i) = -4 | v(j) = 1 |  -3 | d(t) = -4 | needed = 4 <-> 2 | nums = -4 1 2 sum = -1 best_d=2
        
        // i=1 | j=2 | v(i) = -1 | v(j) = 1 | 0 | d(t) = -1 | needed = 1 <-> 2 | nums = -1 1 2 sum = 2 best_d=1

        let mut nums_sorted: Vec<i32> = nums.iter().copied().collect();
        nums_sorted.sort();
        
        let mut best_sum = i32::MAX;

        for i in 0..nums_sorted.len()-2 {
            for j in i+1..nums_sorted.len()-1{
                let two_sum = nums_sorted[i] + nums_sorted[j];
                // let distance_to_target_left = two_sum - target;
                // let needed = -distance_to_target_left;

                for z in j+1..nums_sorted.len(){
                    if (i32::abs(best_sum-target) > i32::abs(two_sum + nums_sorted[z]-target)){
                        best_sum = two_sum + nums_sorted[z]
                    }
                }
            }
        }
        
        return best_sum;

    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);

        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0)
    }
}
