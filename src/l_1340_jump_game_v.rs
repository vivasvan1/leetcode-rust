// https://leetcode.com/problems/jump-game-v/description/
//

use std::collections::HashMap;
use std::io::{self, Write}; // Put this at the top of your file
pub struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut max_jump_from_index_map: HashMap<usize, i32> = HashMap::new();

        for i in 0..arr.len() {
            max_jump_from_index_map.insert(i, -1);
        }

        fn stimulate_at_i(i: usize, arr: &[i32], d: usize, cache: &mut HashMap<usize, i32>) -> i32 {
            if cache.contains_key(&i) && cache[&i] != -1 {
                return cache[&i];
            }

            println!("i: {:?}, arr: {:?}, d: {:?}", i, arr, d);
            io::stdout().flush().unwrap();
            let i_minus_d_mini = if i < d { 0 } else { i - d };
            let i_plus_d_maxi = if i + d >= arr.len() {
                arr.len()
            } else {
                i + d + 1
            };
            println!(
                "i_minus_d_mini:{:?} i_plus_d_maxi:{:?}",
                i_minus_d_mini, i_plus_d_maxi
            );
            io::stdout().flush().unwrap();
            let mut i_minus_d_mini_final = i_minus_d_mini;
            let mut i_plus_d_maxi_final = i_plus_d_maxi;

            for i_temp in i_minus_d_mini..i_plus_d_maxi {
                if i_temp == i {
                    continue;
                }
                if arr[i_temp] >= arr[i] && i_temp < i {
                    // println!(
                    //     "arr[i_temp]: {:?}, i_temp:{:?}, arr[i]: {:?}",
                    //     arr[i_temp], i_temp, arr[i]
                    // );
                    i_minus_d_mini_final = i_temp + 1;
                }
                if arr[i_temp] >= arr[i] && i_temp > i {
                    // println!(
                    //     "arr[i_temp]: {:?}, i_temp:{:?}, arr[i]: {:?}",
                    //     arr[i_temp], i_temp, arr[i]
                    // );
                    i_plus_d_maxi_final = i_temp;
                    break;
                }
            }

            if i_plus_d_maxi_final == i_minus_d_mini_final {
                return 1;
            }

            let mut best_sub_jump = 0;
            println!(
                "i_minus_d_mini_final:{:?}..i_plus_d_maxi_final:{:?}",
                i_minus_d_mini_final, i_plus_d_maxi_final
            );
            io::stdout().flush().unwrap();
            for i_temp in i_minus_d_mini_final..i_plus_d_maxi_final {
                if i_temp == i {
                    continue;
                }
                println!("{:?}, Calling stimulate_at_i at {:?}", i, i_temp);
                io::stdout().flush().unwrap();
                let sub_jump_at_i = stimulate_at_i(i_temp, arr, d, cache);
                if sub_jump_at_i > best_sub_jump {
                    best_sub_jump = sub_jump_at_i;
                }
            }

            cache.insert(i, best_sub_jump + 1);

            println!("Found best_sub_jump at {:?} = {:?}", i, best_sub_jump);

            return best_sub_jump + 1;
            // println!("arr: {:?}, d:{:?}", arr, d);
            // println!(
            //     "i:{:?}, i_minus_d_mini:{:?}, i_plus_d_maxi{:?}, i_minus_d_mini_final:{:?}, i_plus_d_maxi_final:{:?}",
            //     i, i_minus_d_mini, i_plus_d_maxi, i_minus_d_mini_final, i_plus_d_maxi_final
            // )
        }
        let mut best = 0;
        for ith_element in 0..arr.len() {
            let ith_best =
                stimulate_at_i(ith_element, &arr, d as usize, &mut max_jump_from_index_map);
            if ith_best > best {
                best = ith_best;
            }
        }

        return best;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    }
}
