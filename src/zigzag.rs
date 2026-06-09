use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let length = s.len();

        let mut result = String::new();
        let mut char_map: HashMap<i32, Vec<char>> = HashMap::new();
        let mut temp = 1;
        let mut is_inc = true;
        for c in s.chars() {
            match char_map.get_mut(&temp){
                Some(list) => {
                    list.push(c);
                }
                None => {char_map.insert(temp, vec![c]);}
            }
            if temp == num_rows {
                is_inc = false;
                temp -= 1;
            } else if temp == 1 {
                is_inc = true;
                temp += 1;
            } else if is_inc {
                temp += 1;
            } else {
                temp -= 1;
            }
        }
        println!("{:?}", char_map);

        for (i,v) in &char_map {
            println!("{:?}",i);
            println!("{:?}",v);
        }
        return String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(Solution::convert("".to_string(), 1), "".to_string());
    }

    #[test]
    fn test_one_character() {
        assert_eq!(Solution::convert("a".to_string(), 1), "a".to_string());
    }

    #[test]
    fn test_one_character_per_row() {
        assert_eq!(
            Solution::convert("a".to_string(), 2),
            "a\na".to_string()
        );
    }

    #[test]
    fn test_two_characters_per_row() {
        assert_eq!(
            Solution::convert("ab".to_string(), 2),
            "ab\nab".to_string()
        );
    }

    #[test]
    fn test_three_characters_per_row() {
        assert_eq!(
            Solution::convert("abc".to_string(), 2),
            "abc\nabc".to_string()
        );
    }

    #[test]
    fn test_four_characters_per_row() {
        assert_eq!(
            Solution::convert("abcd".to_string(), 2),
            "abcd\nabcd".to_string()
        );
    }
}
