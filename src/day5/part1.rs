use crate::day_input_to_string;

#[derive(Debug, Clone)]
struct Starter {
    pub name: String,
    pub values: Vec<u32>,
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
    List(Vec<u32>),
    Transformer(Transformations),
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct Transformations {
    pub destination: u32,
    pub source: u32,
    pub range: u32,
}

impl Transformer {
    pub fn transform(&self, mut input: u32) -> u32 {
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
        let destination = inputs[0].parse::<u32>().unwrap();
        let source = inputs[1].parse::<u32>().unwrap();
        let range = inputs[2].parse::<u32>().unwrap();
        return Body::Transformer(Transformations {
            destination,
            source,
            range,
        });
    } else {
        let list = inputs
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        return Body::List(list);
    }
}

#[allow(unused)]
pub fn solve_day_5_part_part1() {
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

    let mut results: Vec<u32> = Vec::new();

    for start in starter.values {
        let mut value = start;
        for transformer in transformers.clone() {
            value = transformer.transform(value);
        }
        results.push(value)
    }

    
    println!("Day 5 part part1 solution: {}", results.iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::solve_day_5_part_part1;

    #[test]
    fn test_solve_day_5_part_part1() {
        solve_day_5_part_part1();
    }
}
