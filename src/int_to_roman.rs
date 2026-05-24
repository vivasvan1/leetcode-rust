// https://leetcode.com/problems/integer-to-roman/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut current_decimal: i32 = 10;
        let mut numc = num;
        let mut decimals: Vec<i32> = Vec::new();

        while numc != 0 {
            let v = numc % current_decimal;
            decimals.insert(0, v / (current_decimal / 10));
            numc = numc - v;
            current_decimal = current_decimal * 10;
        }
        println!("{:?}", decimals);
        let int_to_roman_letter_map: HashMap<i32, Vec<(i32, &str)>> = HashMap::from([
            (3, Vec::from([(5000, "_"), (1000, "M")])),
            (2, Vec::from([(500, "D"), (100, "C")])),
            (1, Vec::from([(50, "L"), (10, "X")])),
            (0, Vec::from([(5, "V"), (1, "I")])),
        ]);

        // 3499 - MMMCDXCIX

        let mut result: String = String::new();

        for i in 0..decimals.len() {
            let curr_d = decimals.len() - i - 1;
            let tenth_place = 10_i32.pow(curr_d as u32);

            let curr_value = decimals.get(i).expect("Index out of bound");

            let mut curr_value_times_10s =
                decimals.get(i).expect("Index out of bound") * tenth_place;
            println!(
                "curr_d: {:?} tenth_place: {:?} curr_value: {:?} curr_value_times_10s: {:?}",
                curr_d, tenth_place, curr_value, curr_value_times_10s
            );
            while curr_value_times_10s > 0 {
                
                if *curr_value == 4 {
                    if curr_d == 2 {
                        result.push_str("CD");
                    } else if curr_d == 1 {
                        result.push_str("XL");
                    } else if curr_d == 0 {
                        result.push_str("IV");
                    }
                    break;
                }

                if *curr_value == 9 {
                    if curr_d == 2 {
                        result.push_str("CM");
                    } else if curr_d == 1 {
                        result.push_str("XC");
                    } else if curr_d == 0 {
                        result.push_str("IX");
                    }
                    break;
                }
                println!(
                    "curr_value_times_10s: {:?}, tenth_place: {:?}, result: {:?}",
                    curr_value_times_10s, tenth_place, result
                );
                if curr_value_times_10s >= 5 * tenth_place {
                    let (amount, letter) = int_to_roman_letter_map
                        .get(&(curr_d as i32))
                        .expect("Index out of bound")
                        .get(0)
                        .expect("Index out of bound");

                    curr_value_times_10s -= amount;
                    result.push_str(letter);
                } else {
                    let (amount, letter) = int_to_roman_letter_map
                        .get(&(curr_d as i32))
                        .expect("Index out of bound")
                        .get(1)
                        .expect("Index out of bound");
                    curr_value_times_10s -= amount;
                    result.push_str(letter);
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_string());
    }
}
