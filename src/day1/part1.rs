use crate::day_input_to_string;

#[allow(unused)]
pub fn solve_day_1_part_1() {
    let contents = day_input_to_string!();
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
    println!("Day 1 part 1 solution: {}", coords);
}

#[cfg(test)]
mod tests {
    use super::solve_day_1_part_1;

    #[test]
    fn test_solve_day_1_part_1() {
        solve_day_1_part_1();
    }
}
