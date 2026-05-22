use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_copied: Vec<i32> = nums.iter().copied().collect();
        nums_copied.sort();
        let mut nums_sorted: Vec<i32> = nums_copied.iter().copied().collect();
 
        let mut hashmap: HashMap<i32, usize> = HashMap::new();

        for (i, v) in nums_sorted.iter().enumerate() {
            hashmap.insert(*v, i);
        }

        println!("{:?}", hashmap);
        
        // -4, -1, -1, 0, 1, 2
        //      _         

        let mut i  = 0;
        let mut j = i+1;

        while i < nums_sorted.len()-2 {
            if nums.get(i).expect("out of bound") > &0{
                break;
            }
            while j < nums_sorted.len() - 1 {
                let z_needed = -nums_sorted.get(i).expect("oob") -nums_sorted.get(j).expect("oob");
                print!("{:?}",hashmap.contains_key(&z_needed));
            } 
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
