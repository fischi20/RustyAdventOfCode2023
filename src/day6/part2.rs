use std::ops::Range;

use crate::day_input_to_string;

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64
}

fn compute_distance_bigger_record(time: f64, record_distance: f64)-> Range<u64>{
    let delta = (time*time)-(4.0*-1.0*-record_distance);
    let x1 = (((-time + delta.sqrt())/-2.)+1.).floor() as u64; //+1 and then floor is to account for the fact that the results are inclusive, but they shouldn't be
    let x2 = (((-time- delta.sqrt())/-2.)-1.).ceil() as u64; // -1 and then ceil is to account for the fact that the results are inclusive, but they shouldn't be

    x1..x2
}

fn parse_records(input: impl Into<String>)->Vec<Record> {
    let inputs:String = input.into();
    let mut records: Vec<Record> = Vec::new();

    let lines = inputs.lines().collect::<Vec<_>>();
    let times = lines[0].split(":").collect::<Vec<_>>()[1].split_ascii_whitespace().collect::<Vec<_>>().join("");
    let distances = lines[1].split(":").collect::<Vec<_>>()[1].split_ascii_whitespace().collect::<Vec<_>>().join("");

    println!("times: {}", times);
    println!("distances: {}", distances);


        records.push(Record {
            time: times.parse::<u64>().unwrap(),
            distance: distances.parse::<u64>().unwrap()
        });
    

    records
}

#[allow(unused)]
pub fn solve_day_6_part_part2() {
    let contents = day_input_to_string!();
    let records = parse_records(contents);

    let mut results = Vec::new();

    for record in records {
        let range = compute_distance_bigger_record(record.time as f64, record.distance as f64);
        results.push(range.end - range.start +1);
    }

    println!("Day 6 part part2 solution: {}", results.iter().product::<u64>());
}

#[cfg(test)]
mod tests {
    use super::solve_day_6_part_part2;

    #[test]
    fn test_solve_day_6_part_part2() {
        solve_day_6_part_part2();
    }
}
