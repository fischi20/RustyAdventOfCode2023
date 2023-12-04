use crate::day_input_to_string;

#[derive(Debug)]
struct Picks {
    red: u32,
    green: u32,
    blue: u32,
}

#[allow(unused)]
#[derive(Debug)]
struct Game {
    game_id: u32,
    picks: Vec<Picks>,
}

fn extract_game_id_from_header(header: impl Into<String>) -> u32 {
    let input: String = header.into();
    let base_split: Vec<String> = input.split(":").map(|s| s.to_string()).collect();
    let header = base_split[0].clone();
    let game_id = header.split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    return game_id;
}

fn extract_picks_from_round(round: impl Into<String>) -> Picks {
    let round: String = round.into();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for pick in round.split(",") {
        let split_pick: Vec<&str> = pick.trim().split(" ").collect();
        let amount = split_pick[0].parse::<u32>().unwrap();
        let color = split_pick[1];
        match color {
            "red" => red = amount,
            "green" => green = amount,
            "blue" => blue = amount,
            _ => panic!("Invalid color: {}", color),
        }
    }

    return Picks { red, green, blue };
}

fn parse_game(line: impl Into<String>) -> Game {
    let input: String = line.into();
    let base_split: Vec<String> = input.split(":").map(|s| s.trim().to_string()).collect();
    let header = base_split[0].clone();
    let picks = base_split[1].clone();

    let mut game_picks: Vec<Picks> = vec![];

    let game_id = extract_game_id_from_header(header);

    for round in picks.split(";") {
        game_picks.push(extract_picks_from_round(round.trim()));
    }

    return Game {
        game_id,
        picks: game_picks,
    };
}

#[allow(unused)]
pub fn solve_day_2_part_part2() {
    let contents = day_input_to_string!();
    let mut total_power = 0;

    for line in contents.lines() {
        let game = parse_game(line);
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for pick in game.picks {
            if pick.red > max_red {
                max_red = pick.red;
            }
            if pick.green > max_green {
                max_green = pick.green;
            }
            if pick.blue > max_blue {
                max_blue = pick.blue;
            }
        }

        total_power += max_red * max_green * max_blue;
    }

    println!("Day 2 part part2 solution: {}", total_power);
}

#[cfg(test)]
mod tests {
    use super::solve_day_2_part_part2;

    #[test]
    fn test_solve_day_2_part_part2() {
        solve_day_2_part_part2();
    }
}
