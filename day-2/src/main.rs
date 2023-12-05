use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("The Sum of the IDs is: {}", part_one("input"));
    println!("The Sum of power is: {}", part_two("input"));
}

fn part_one(input: &str) -> usize {
    let config = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum = 0;

    for line in read_to_string(input).unwrap().lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let header: &str = parts.get(0).unwrap();
        let body: &str = parts.get(1).unwrap();
        let sets = body.split(";").collect::<Vec<_>>().join(",");

        let mut pass_min_config_values = true;

        for set in sets.split(",").collect::<Vec<_>>() {
            let values = set.trim().split(" ").collect::<Vec<_>>();
            let color = values.get(1).unwrap();
            let cubes = values.get(0).unwrap().parse::<usize>().unwrap();

            if config.get(color).unwrap() < &cubes {
                pass_min_config_values = false;
            }
        }

        if pass_min_config_values {
            let header_parts = header.split(" ").collect::<Vec<&str>>();
            sum += header_parts[1].parse::<usize>().unwrap();
        }
    }

    return sum;
}

fn part_two(input: &str) -> usize {
    let mut sum_of_powers = 0;

    for line in read_to_string(input).unwrap().lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let body: &str = parts.get(1).unwrap();
        let sets = body.split(";").collect::<Vec<_>>().join(",");
        let mut max_colors: HashMap<&str, usize> = HashMap::new();

        for set in sets.split(",").collect::<Vec<_>>() {
            let pair = set.trim().split(" ").collect::<Vec<_>>();

            if max_colors.contains_key(pair[1]) {
                if max_colors.get(pair[1]).unwrap() < &pair[0].clone().parse::<usize>().unwrap() {
                    *max_colors.get_mut(pair[1]).unwrap() = pair[0].parse::<usize>().unwrap(); 
                }
            } else {
                max_colors.insert(pair[1], pair[0].parse::<usize>().unwrap());
            }
        }

        sum_of_powers += max_colors.iter().fold(1, |acc, element| {
            acc * element.1
        });
    }

    return sum_of_powers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("input_test_part_1"), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("input_test_part_2"), 2286);
    }
}
