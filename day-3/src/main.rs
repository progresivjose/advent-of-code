use std::fs::read_to_string;


fn main() {
    println!("The result is {}", sum_the_numbers("input"));    
}

fn sum_the_numbers(input: &str) -> usize {
    let mut adjacent_numbers: Vec<usize> = vec![];
    let data = read_to_string(input).unwrap();
    let mut matrix: Vec<&str> = vec![];
    let directions: Vec<(i64, i64)> = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0), 
        (-1, 1), (0, 1), (1, 1)
    ];

    for line in data.lines() {
        matrix.push(line);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sum_the_numbers() {
        assert_eq!(sum_the_numbers("input_test"), 4361);
    }
}
