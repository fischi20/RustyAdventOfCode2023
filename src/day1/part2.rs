use crate::{day_input_to_string, hashmap};

#[allow(unused)]
pub fn solve_day_1_part_2() {
    let mut contents = day_input_to_string!();

    let replace_map = hashmap! {
        "one"=>"1",
        "two"=>"2",
        "three"=>"3",
        "four"=>"4",
        "five"=>"5",
        "six"=>"6",
        "seven"=>"7",
        "eight"=>"8",
        "nine"=>"9"
    };

    for pattern in replace_map.keys() {
        contents = contents.replace(pattern, replace_map.get(pattern).unwrap());
    }

    let mut coords: u32 = 0;
    for line in contents.lines() {
        let mut cal_val: Option<u32> = None;
        let mut last_val: u32 = 0;
        for char in line.chars() {
            if char.is_numeric() {
                last_val = char.to_digit(10).unwrap();
                if cal_val.is_none() {
                    cal_val = Some(last_val * 10);
                }
            }
        }

        coords += cal_val.unwrap_or(0) + last_val;
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
