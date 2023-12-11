use std::{fs::read_to_string, collections::HashMap};

fn main() {
    println!("the sum of points {}", get_the_points("input"));
    println!("the number of cards {}", get_the_number_of_cards("input"));
}

fn get_the_points(input: &str) -> usize {
    let data = read_to_string(input).unwrap();
    let mut sum = 0;

    for card in data.lines().collect::<Vec<&str>>().iter() {
        let mut points = 0;
        let (winning_numbers, my_numbers) = card
            .split_once(":")
            .unwrap()
            .1
            .split_once("|")
            .unwrap();

        let winning: Vec<&str> = winning_numbers.trim().split(" ").filter(|val| val.len() > 0).collect();
        let numbers: Vec<&str> = my_numbers.trim().split(" ").filter(|val| val.len() > 0).collect();

        for number in numbers {
            if winning.contains(&number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }  
        }

        sum += points;
    }

    sum
}

fn get_the_number_of_cards(input: &str) -> usize {
    let data = read_to_string(input).unwrap();
    let mut cards_coincidences: HashMap<String, usize> = HashMap::new(); 
    let mut cards_amount: HashMap<String, usize> = HashMap::new();

    for card in data.lines().collect::<Vec<&str>>().iter() {
        let (card_name, numbers) = card
            .split_once(":")
            .unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once("|").unwrap(); 
        let winning: Vec<&str> = winning_numbers.trim().split(" ").filter(|val| val.len() > 0).collect();
        let numbers: Vec<&str> = my_numbers.trim().split(" ").filter(|val| val.len() > 0).collect();
        let coincidences: Vec<&str> = numbers.iter().filter(|number| winning.contains(*number)).map(|number| *number).collect();

        cards_coincidences.entry(card_name.to_string()).or_insert(coincidences.len());
        cards_amount.entry(card_name.to_string()).or_insert(1);
    }
        
    for index in 1..=cards_amount.len() {
        let key = get_key(index);
        let conincidences = cards_coincidences.get(&key).unwrap();
        let current_copies = cards_amount.get(&key).unwrap();

        for _ in 1..=*current_copies {
            for coinc_index in 1..=*conincidences {
                let coinc_key = get_key(index + coinc_index);

                *cards_amount.get_mut(&coinc_key).unwrap() += 1;
            }
        }
    } 

    cards_amount.into_iter().map(|(_, v)| v).sum()
}

fn get_key(index: usize) -> String {
    if index < 10 {
        return format!("Card   {}", index);
    } else if index < 100 {
        return format!("Card  {}", index);
    } else {
        return format!("Card {}", index);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_get_the_points() {
        assert_eq!(get_the_points("input_test"), 13);
    }

    #[test]
    fn it_should_get_the_number_of_cards() {
        assert_eq!(get_the_number_of_cards("input_test"), 30);
    }
}
