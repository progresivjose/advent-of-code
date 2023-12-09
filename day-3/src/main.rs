use std::{fs::read_to_string, collections::{HashMap, HashSet}};
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Number {
    start_pos: Position,
    end_pos: Position,
    number: u32
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Gear {
    first_number_pos: Position,
    second_number_pos: Position,
    character: char
}

fn main() {
    println!("The result of the sum of numbers is {}", sum_the_numbers("input"));    
    println!("The result of the sum of ratios is {}", find_the_ratio("input"));    
}

fn sum_the_numbers(input: &str) -> usize {
    let mut adjacent_numbers: Vec<usize> = vec![];
    let data = read_to_string(input).unwrap();
    let matrix: Vec<&str> = data.lines().collect();
    let directions: Vec<(i64, i64)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0), 
        (-1, 1), (0, 1), (1, 1)
    ];

    for (rindex, row) in matrix.iter().enumerate() {
        let characters = row.chars().collect::<Vec<_>>();
        let numbers_pos = get_numbers_pos(&characters);
        let line = row.to_string();

        for (start, end) in &numbers_pos {
            for (x, y) in directions.iter() {
                if y + (rindex as i64) < 0 || y + (rindex as i64) > (matrix.len() as i64) - 1{
                    continue;
                }

                if x + (*start as i64) < 0 || x + (*end as i64) > (row.len() as i64) - 1{
                    continue;
                }

                let index = rindex as i64 + y;

                if index < 0 || index > row.len() as i64 -1 {
                    continue;
                }

                let lookup_characters = matrix[index as usize].chars().collect::<Vec<_>>();

                if is_symbol(lookup_characters[(x + (*start as i64)) as usize]) || is_symbol(lookup_characters[(x + (*end as i64)) as usize]) {
                    if let Some(number) = line.get(*start..*end+1) {
                        adjacent_numbers.push(number.parse::<usize>().unwrap());
                        break;
                    }
                }

            }
        } 
    }

    let mut result: usize = 0;

    adjacent_numbers.iter().for_each(|number| {
        result += number;
    });

    return result;

}

fn is_dot(character: char) -> bool {
    character == '.'
}

fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && !is_dot(character)
}


fn get_numbers_pos(text: &Vec<char>) -> Vec<(usize, usize)> {
    let mut start_pos = None;
    let mut positions: Vec<(usize, usize)> = vec![];

    for (index, character) in text.iter().enumerate() {
        if character.is_ascii_digit() {
            if start_pos.is_none() {
                start_pos = Some(index);
            }
        } else {
            if let Some(start) = start_pos {
                positions.push((start, index - 1));
                start_pos = None;
            }
        }
    }

    if let Some(start) = start_pos {
        positions.push((start, text.len() - 1));
    }

    return positions;
} 

fn find_the_ratio(input: &str) -> u32 {
        let engine = read_to_string(input)
                .unwrap()
                .trim()
                .lines()
                .map(|line| line.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();


    let gears = engine
        .iter()
        .enumerate()
        .map(|(row_idx, row)| 
            row
                .iter()
                .enumerate()
                .filter(|(_, character)| **character == '*')
                .filter(|(col_idx, _)| is_adjacent_gear(&row_idx, &col_idx, &engine))
                .map(move |(col_idx, _)| 
                    (row_idx, col_idx)
                )
                .unique()
                .collect::<Vec<_>>()
        )
        .flatten()
        .collect::<Vec<_>>();


    let mut numbers: Vec<Number> = vec![];
    for (row_idx, row) in engine.iter().enumerate() {
        let mut number = 0;
        for (col_idx, character) in row.iter().enumerate() {
            if !character.is_ascii_digit() {
                continue;
            }

            number *= 10;
            number += character.to_digit(10).unwrap(); 

            if col_idx + 1 >= engine[row_idx].len() || !engine[row_idx][col_idx + 1].is_ascii_digit() {
                let start_col_idx = col_idx - (number.to_string().len() - 1);
                numbers.push(Number { 
                    start_pos: { Position { row: row_idx, col: start_col_idx } },
                    end_pos: { Position { row: row_idx, col: col_idx } },
                    number
                });

                number = 0;
            }
        }
    }

    let mut adjacent_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new(); 
    for (row_idx, col_idx) in gears {
        for number in &numbers {
            if is_adjacent_number(number, &row_idx, &col_idx, &engine) {
                adjacent_numbers.entry((row_idx, col_idx)).or_default().push(number.number);
            }
        }
    }

    adjacent_numbers
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers[0] * numbers[1])
        .sum()

}

fn is_adjacent_gear(row_idx: &usize, col_idx: &usize, engine: &Vec<Vec<char>>) -> bool {
    let directions = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let mut is_adjacent = false;

    for (offset_x, offset_y) in directions {
        let lookup_row = *row_idx as i32 + offset_y;
        let lookup_col = *col_idx as i32 + offset_x;

        if lookup_row < 0 || lookup_row >= engine.len() as i32
            || lookup_col < 0 || lookup_col >= engine[*row_idx].len() as i32 {
            continue;
        }

        if engine[lookup_row as usize][lookup_col as usize].is_ascii_digit() {
            is_adjacent = true;
        }
    }

    is_adjacent
}

fn is_adjacent_number(number: &Number, row_idx: &usize, col_idx: &usize, engine: &Vec<Vec<char>>) -> bool {
    let directions = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let mut is_adjacent = false;

    for (offset_x, offset_y) in directions {
        let start_lookup_row = number.start_pos.row as i32 + offset_y;
        let start_lookup_col = number.start_pos.col as i32 + offset_x;
        let end_lookup_row = number.end_pos.row as i32 + offset_y;
        let end_lookup_col = number.end_pos.col as i32 + offset_x;

        if start_lookup_row < 0 || start_lookup_row >= engine.len() as i32
            || start_lookup_col < 0 || start_lookup_col >= engine[*row_idx].len() as i32 {
            continue;
        }
        
        if end_lookup_row < 0 || end_lookup_row >= engine.len() as i32
            || end_lookup_col < 0 || end_lookup_col >= engine[*row_idx].len() as i32 {
            continue;
        }

        if (start_lookup_row as usize == *row_idx && start_lookup_col as usize == *col_idx)
            || (end_lookup_row as usize == *row_idx && end_lookup_col as usize == *col_idx) {
            is_adjacent = true;
        }
    }

    is_adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sum_the_numbers() {
        assert_eq!(sum_the_numbers("input_test"), 4361);
    }

    #[test]
    fn it_shoud_find_the_gear_ratio() {
        assert_eq!(find_the_ratio("input_test"), 848524);
        assert_eq!(find_the_ratio("input"), 79844424);
    }
}
