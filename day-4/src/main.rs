use std::fs::read_to_string;

fn main() {
     println!("the sum of points {}", get_the_points("input"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_get_the_points() {
        assert_eq!(get_the_points("input_test"), 13);
    }

}
