use crate::day_input_to_string;

#[allow(unused)]
#[derive(Debug, Clone)]
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

    let game_id = header
        .split(" ")
        .filter(|el| el.len() > 0)
        .collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();

    let split2: Vec<_> = game.split("|").collect();

    let winning = split2[0]
        .split(" ")
        .filter(|el| el.len() > 0)
        .map(|el| el.parse().unwrap())
        .collect();
    let having = split2[1]
        .split(" ")
        .filter(|el| el.len() > 0)
        .map(|el| el.parse().unwrap())
        .collect();

    return Game {
        game_id,
        winning,
        having,
    };
}

#[allow(unused)]
pub fn solve_day_4_part_part2() {
    let contents = day_input_to_string!();
    let mut games:Vec<(u32, u32)> = Vec::new();
    let mut total = 0;

    contents.lines().enumerate().for_each(|(i, line)| {
        let game = parse_games(line.to_string());
        let mut game_hits = 0u32;

        for el in game.having.clone() {
            if game.winning.contains(&el) {
                game_hits += 1;
            }
        }

        games.push((game_hits, 1));
    });

    for index in 0..games.len() {
            let (hits, times) = games[index];
            total += 1 * times;
            // increment hits of all the coming cards
            for i in 1..=hits as usize {
                if i < games.len() {
                    games[index+i].1 += 1*times; 
                }
            }
        
    }

    println!("Day 4 part part2 solution: {}", total);
}

#[cfg(test)]
mod tests {
    use super::solve_day_4_part_part2;

    #[test]
    fn test_solve_day_4_part_part2() {
        solve_day_4_part_part2();
    }
}
