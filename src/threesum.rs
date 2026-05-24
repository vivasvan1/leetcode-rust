use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_copied: Vec<i32> = nums.iter().copied().collect();
        nums_copied.sort();
        let nums_sorted: Vec<i32> = nums_copied.iter().copied().collect();
 
        let mut hashmap: HashMap<i32, usize> = HashMap::new();

        for (i, v) in nums_sorted.iter().enumerate() {
            hashmap.insert(*v, i);
        }

        println!("{:?}", hashmap);
        // -4, -1, -1, 0, 1, 2
        // i = -4, j = -1 k_needed = 5
        //
        // i = -4  j = -1 k_needed = 5
        //
        // i = -4  j = 0  k_needed = 4
        //
        // i = -4  j = 1  k_needed = 3
        //
        // i = -1 j = -1 k_needed =2    [-1, -1, 2] 1,2,4 -> 1,3,5 -> 1,4,5
        //
        // i = -1 j = 0 k_needed = 1   [-1, 0, 1]  1,3,4 -> 1,3 -> 1,4
        //
        // i = -1 j = 1 k_needed = 0   ignore because k<j    1,5,3 -> 1,5,3 -> 1,6,3 -> 2,6,3 -> 3,6,3 -> 3,4
        //
        // i = 0 j = 1 k_needed = -1   ignore 3,4 -> 3,5
        //
        //
        //
        //  
        //
        //
        // 
        let mut i  = 0;
        let mut j;
        let mut f: Vec<Vec<i32>> = vec![];
        while i < nums_sorted.len()-2 {
            println!("{:?}", i);
            if nums.get(i).expect("out of bound") > &0{
                break;
            }
            j = i+1;
            while j < nums_sorted.len() - 1 {
                let z_needed = -nums_sorted.get(i).expect("oob") -nums_sorted.get(j).expect("oob");
                if hashmap.contains_key(&z_needed) && hashmap.get(&z_needed).expect("oob") >&j {
                    f.push(vec![*nums_sorted.get(i).expect("oob"),*nums_sorted.get(j).expect("oob"),z_needed]);
                }
                j = *hashmap.get(&nums_sorted[j]).expect("oob");
                j+=1;
            }
            i = *hashmap.get(nums_sorted.get(i).expect("oob")).expect("oob");
            i+=1;
        }
        return f;
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
