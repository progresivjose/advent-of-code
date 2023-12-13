use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    println!("The Lowest Location: {}", find_lowest_location("input"));
}

fn find_lowest_location(input: &str) -> usize {
    let data = read_to_string(input).unwrap();
    let lines: Vec<&str> = data.lines().collect::<Vec<_>>();

    let seeds = lines[0]
        .split(":")
        .into_iter()
        .collect::<Vec<_>>()[1]
        .trim()
        .split(" ")
        .into_iter()
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps: HashMap<&str, Vec<Vec<usize>>> = HashMap::new();
    let mut key: &str = "";

    for index in 1..lines.len() {
        if lines[index] == "" {
            continue;
        }

        if let Some(_found) = lines[index].find("map") {
            key = lines[index].split(" ").into_iter().collect::<Vec<_>>()[0];
            continue;
        }

        let numbers = lines[index].trim().split(" ").into_iter().map(|number| {
            number.parse::<usize>().unwrap()
        }).collect::<Vec<_>>();

        maps.entry(key).or_default().push(numbers);
    }

    let soils = get_paths(&maps, "seed-to-soil", &seeds);
    let ferts = get_paths(&maps, "soil-to-fertilizer", &soils.clone().into_iter().map(|(_, v)| v).collect::<Vec<_>>());
    let water = get_paths(&maps, "fertilizer-to-water", &ferts.clone().into_iter().map(|(_, v)| v).collect::<Vec<_>>());
    let light = get_paths(&maps, "water-to-light", &water.clone().into_iter().map(|(_, v)| v).collect::<Vec<_>>());
    let temp = get_paths(&maps, "light-to-temperature", &light.clone().into_iter().map(|(_, v)| v).collect::<Vec<_>>());
    let humidity = get_paths(&maps, "temperature-to-humidity", &temp.clone().into_iter().map(|(_, v)| v).collect::<Vec<_>>());
    let location = get_paths(&maps, "humidity-to-location", &humidity.clone().into_iter().map(|(_, v)| v).collect::<Vec<_>>());
    let mut locations = location.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    locations.sort();

    *locations.get(0).unwrap_or(&0)
}

fn get_paths(dataset: &HashMap<&str, Vec<Vec<usize>>>, key: &str, prev_paths: &Vec<usize>) -> HashMap<usize, usize> {
    let mut selected_paths: HashMap<usize, usize> = HashMap::new();
    let paths = dataset.get(key).unwrap();

    for prev in prev_paths {
        for path in paths {
            if prev < &path[1] {
                continue;
            }

            let path_sum = path[1] + path[2];

            if &path_sum < prev {
                continue;
            }

            let rest = prev - path[1];

            *selected_paths.entry(*prev).or_default() = path[0] + rest; 
        }
    }

    for prev in prev_paths {
        if selected_paths.get(&prev).is_none() {
            *selected_paths.entry(*prev).or_default() = prev.clone();
        }
    }

    selected_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lowest_location() {
        assert_eq!(find_lowest_location("input_test"), 35);
    }
}
