use std::collections::HashMap;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let length = s.len();

        let mut result = String::new();
        let mut char_map: HashMap<i32, Vec<char>> = HashMap::new();
        let mut temp = 1;
        let mut is_inc = true;
        for c in s.chars() {
            match (char_map.get_mut(&temp)){
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
            println!("{:?}",i)
            println!("{:?}",v)
        }
        return String::new()
    }
}
