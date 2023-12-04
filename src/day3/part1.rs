use crate::day_input_to_string;

type CharGrid<'a> = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone)]
struct Position {
    pub line: usize,
    pub col: usize,
}

fn has_adiecent_symbols(
    val: impl Into<String>,
    position: Position,
    content: &CharGrid,
) -> bool {
    let value: String = val.into();

    let start_line: usize = (position.line as i32 - 1).max(0) as usize;
    let start_col: usize = (position.col as i32 - 1).max(0) as usize;
    let end_line: usize = (position.line + 1).min(content.len()-1) as usize;
    let end_col: usize = (position.col + value.len()).min(content[0].len()-1) as usize;

    for line in start_line..=end_line {
        for col in start_col..=end_col {
            if content[line][col].is_ascii_punctuation() && content[line][col] != '.' {
                return true;
            }
        }
    }

    false
}

fn get_next_number(position: Position, content: &CharGrid) -> Option<(String, Position, Position)> {
    let mut value: String = "".into();
    let mut start_position = None;
    let mut start_col = position.col;
    for line in position.line..content.len() {
        for col in start_col..content[line].len() {
            if content[line][col].is_numeric() {
                if start_position.is_none() {
                    start_position = Some(Position { line, col })
                }
                if start_position.is_some() {
                    value.push(content[line][col]);
                }
            } else {
                if start_position.is_some() {
                    return Some((value, start_position.unwrap(), Position { line, col }));
                }
            }
        }
        start_col = 0;
        if start_position.is_some() {
            // if new line is entered
            return Some((
                value,
                start_position.unwrap(),
                Position {
                    line: line + 1,
                    col: start_col,
                },
            ));
        }
    }
    None
}

#[allow(unused)]
pub fn solve_day_3_part_part1() {
    let contents = day_input_to_string!();
    let lines: Vec<_> = contents.lines().collect();
    let char_lines: CharGrid = lines.iter().map(|line| line.chars().collect()).collect();
    let mut position = Position { line: 0, col: 0 };

    let mut total = 0;
    let mut number_amount = 0;

    loop {
        match get_next_number(position, &char_lines) {
            Some((val, start_position, end_position)) => {
                if has_adiecent_symbols(&val, start_position, &char_lines) {
                    total += val.parse::<u32>().unwrap();
                    number_amount += 1;
                }
                position = end_position;
            }
            None => {
                break;
            }
        }
    }
    println!("Day 3 part part1 solution: {}", total);
}

#[cfg(test)]
mod tests {
    use super::solve_day_3_part_part1;

    #[test]
    fn test_solve_day_3_part_part1() {
        solve_day_3_part_part1();
    }
}
