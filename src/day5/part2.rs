use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::day_input_to_string;

#[derive(Debug, Clone)]
struct Starter {
    pub name: String,
    pub values: Vec<u64>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct Transformer {
    pub from: String,
    pub to: String,
    pub transformers: Vec<Body>,
}

#[derive(Debug, Clone)]
enum Block {
    Starter(Starter),
    Transformer(Transformer),
}

#[derive(Debug, Clone)]
enum Body {
    List(Vec<u64>),
    Transformer(Transformations),
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct Transformations {
    pub destination: u64,
    pub source: u64,
    pub range: u64,
}

impl Transformer {
    pub fn transform(&self, mut input: u64) -> u64 {
        for t in self.transformers.iter() {
            match t {
                Body::Transformer(t) => {
                    if input >= t.source && input < t.source + t.range {
                        let offset = input - t.source;
                        input = t.destination + offset;
                        break;
                    }
                }
                _ => {}
            }
        }
        input
    }
}

fn parse_transformation(line: impl Into<String>) -> Body {
    let mut line: String = line.into();
    line = line.trim().to_string();

    let inputs = line.split(" ").collect::<Vec<_>>();
    if inputs.len() == 3 {
        let destination = inputs[0].parse::<u64>().unwrap();
        let source = inputs[1].parse::<u64>().unwrap();
        let range = inputs[2].parse::<u64>().unwrap();
        return Body::Transformer(Transformations {
            destination,
            source,
            range,
        });
    } else {
        let list = inputs
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        return Body::List(list);
    }
}

#[allow(unused)]
pub fn solve_day_5_part_part2() {
    let contents = day_input_to_string!();

    let mut blocks = Vec::new();
    for (block_index, block) in contents.split("\r\n\r\n").enumerate() {
        let mut from = String::new();
        let mut to = None;
        let mut body = Vec::new();
        for (index, block) in block.split(":").enumerate() {
            if index == 0 {
                let header = block.split(" ").collect::<Vec<_>>()[0];
                let split_header = header.split("-to-").collect::<Vec<_>>();
                from = split_header[0].to_string();
                to = split_header.get(1).map(|s| s.to_string());
            } else {
                for line in block.trim().lines() {
                    body.push(parse_transformation(line.trim()));
                }
            }
        }
        if (to.is_some()) {
            // insert after the last Block that is either a Starter with name of from or a Transformer with the to field equal to from
            let block = Block::Transformer(Transformer {
                from: from.clone(),
                to: to.unwrap(),
                transformers: body,
            });

            if let Some(index) = blocks.clone().iter().position(|b| match b {
                Block::Starter(s) => s.name.clone() == from.clone(),
                Block::Transformer(t) => t.to.clone() == from.clone(),
            }) {
                blocks.insert(index + 1, block);
            } else {
                blocks.push(block);
            }
        } else {
            let mut temp = vec![Block::Starter(Starter {
                name: from,
                values: body
                    .iter()
                    .map(|b| match b {
                        Body::List(l) => l.clone(),
                        _ => panic!("Expected Body::List"),
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
            })];
            temp.extend(blocks);
            blocks = temp;
        }
    }

    let starter: Starter = blocks
        .iter()
        .find_map(|b| match b {
            Block::Starter(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap();

    let transformers: Vec<Transformer> = blocks
        .iter()
        .filter_map(|b| match b {
            Block::Transformer(t) => Some(t.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut results = Arc::new(Mutex::new(0));

    for (start, length) in starter
        .values
        .iter()
        .zip(starter.values.iter().skip(1))
        .step_by(2)
    {
        let range = *start..(start + length);
        // print the range that we are working with right now
        println!("Range: {:?} -> {:?}", range.start, range.end);
        let progress = Arc::new(Mutex::new(0));

        range.into_par_iter().for_each(|start| {
            let mut value = start;
            for transformer in transformers.clone() {
                value = transformer.transform(value);
            }

            {
               let mut min = results.lock().unwrap();
                if value < *min {
                     *min = value;
                }
            }
            {
                let mut progress = progress.lock().unwrap();
                //print every 1% of progress
                if *progress % (length / 100) == 0 {
                    println!("Progress: {}%", *progress / (length / 100));
                }
                *progress += 1;
            }
        });
    }

    println!(
        "Day 5 part part2 solution: {}",
        results.lock().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::solve_day_5_part_part2;

    #[test]
    fn test_solve_day_5_part_part2() {
        solve_day_5_part_part2();
    }
}
