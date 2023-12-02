use crate::day_input_to_string;

#[derive(Debug)]
struct Picks {
    red: u32,
    green: u32,
    blue: u32,
}

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
pub fn solve_day_2_part_part1() {
    let contents = day_input_to_string!();
    let mut id_sums = 0;
    let red_amount = 12;
    let green_amount = 13;
    let blue_amount = 14;

    'game: for line in contents.lines() {
        let game = parse_game(line);
        for pick in game.picks {
            if pick.red > red_amount || pick.green > green_amount || pick.blue > blue_amount {
                continue 'game;
            }
        }
        id_sums += game.game_id;
    }

    println!("Day 2 part part1 solution: {}", id_sums);
}

#[cfg(test)]
mod tests {
    use super::solve_day_2_part_part1;

    #[test]
    fn test_solve_day_2_part_part1() {
        solve_day_2_part_part1();
    }
}
