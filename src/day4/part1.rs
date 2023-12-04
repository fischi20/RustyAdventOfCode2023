use std::sync::{Mutex, Arc};

use crate::day_input_to_string;
use rayon::prelude::*;

#[allow(unused)]
#[derive(Debug)]
struct Game {
    pub game_id: u32,
    pub winning: Vec<u32>,
    pub having: Vec<u32>,
}

fn parse_games(line: impl Into<String>) -> Game {
    let line: String = line.into();
    let split1: Vec<_> = line.split(":").collect();
    let header = split1[0];
    let game = split1[1];

    let game_id = header.split(" ").filter(|el| el.len() > 0).collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();

    let split2: Vec<_> = game.split("|").collect();

    let winning = split2[0].split(" ").filter(|el| el.len() > 0).map(|el| el.parse().unwrap()).collect();
    let having = split2[1].split(" ").filter(|el| el.len() > 0).map(|el| el.parse().unwrap()).collect();

    return Game {
        game_id,
        winning,
        having,
    };
}

#[allow(unused)]
pub fn solve_day_4_part_part1() {
    let contents = day_input_to_string!();
    let mut total = Arc::new(Mutex::new(0));
    contents
        .lines()
        .enumerate()
        .collect::<Vec<(usize, &str)>>()
        .par_iter()
        .for_each(|(i, line)| {
            let game = parse_games(line.to_string());

            let mut pow = -1;

            for el in game.having {
                if game.winning.contains(&el) {
                    pow += 1;
                }
            }

            if pow >= 0 {
                let mut total = total.lock().unwrap();
                *total += 2u32.pow(pow as u32);
            }

        });

    println!("Day 4 part part1 solution: {}", total.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::solve_day_4_part_part1;

    #[test]
    fn test_solve_day_4_part_part1() {
        solve_day_4_part_part1();
    }
}
