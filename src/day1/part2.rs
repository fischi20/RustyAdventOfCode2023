use std::collections::HashMap;

use crate::{day_input_to_string, hashmap};

#[allow(unused)]
pub fn solve_day_1_part_2() {
    let mut contents = day_input_to_string!();

    let replace_map: HashMap<&str, u32> = hashmap! {
        "one"=>1,
        "two"=>2,
        "three"=>3,
        "four"=>4,
        "five"=>5,
        "six"=>6,
        "seven"=>7,
        "eight"=>8,
        "nine"=>9
    };


    let mut coords: u32 = 0;
    for line in contents.lines() {
        let mut left_val: Option<u32> = None;
        let mut right_val: Option<u32> = None;
        'charIter: for i in 0..line.len() {
            let current_left_char = line.chars().nth(i).unwrap();
            let current_right_char = line.chars().nth(line.len()-(i+1)).unwrap();
            if left_val.is_none() {
                if current_left_char.is_numeric() {
                    left_val = Some(current_left_char.to_digit(10).unwrap());
                } else {
                    // check if one of the replace_map keys is present
                    let remaining_string = line.get(i..).unwrap();
                    for number in replace_map.keys() {
                        if remaining_string.starts_with(number) {
                            left_val = Some(*replace_map.get(number).unwrap());
                        }
                    }
                }
            }

            if right_val.is_none() {
                if current_right_char.is_numeric() {
                    right_val = Some(current_right_char.to_digit(10).unwrap());
                } else {
                    
                    let remaining_string: String = line
                        .get(0..line.len() - (i))
                        .unwrap()
                        .chars()
                        .rev()
                        .collect();



                    for number in replace_map.keys() {
                        let reversed_number: String = number.chars().rev().collect();
                        if remaining_string.starts_with(&reversed_number) {
                            right_val = Some(*replace_map.get(number).unwrap())
                        }
                    }
                }
            }


            
            if right_val.is_some() && left_val.is_some() {
                break 'charIter;
            }
        }

        coords += left_val.unwrap() * 10 + right_val.unwrap();
    }
    println!("Day 1 part 2 solution: {}", coords);
}

#[cfg(test)]
mod tests {
    use super::solve_day_1_part_2;

    #[test]
    fn test_solve_day_1_part_2() {
        solve_day_1_part_2();
    }
}
