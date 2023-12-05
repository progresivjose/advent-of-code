use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("Result for first part: {}", first_part("input"));
    println!("Result for second part: {}", second_part("input"));
}

fn calibrate(data: Vec<(usize, usize)>) -> usize {
    let mut result: usize = 0;

    for (first_digit, last_digit) in data {
        result += (first_digit * 10) + last_digit
    }

    return result;
}

fn first_part(input: &str) -> usize {
    let re = Regex::new(r"\d").unwrap();
    let mut data: Vec<(usize, usize)> = vec![];

    for line in read_to_string(input).unwrap().lines() {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

        let first_digit = matches[0].parse::<usize>().unwrap();
        let last_digit = matches[matches.len() - 1].parse::<usize>().unwrap();

        data.push((first_digit, last_digit));
    }

    return calibrate(data);
}

fn second_part(input: &str) -> usize {
    let mut data: Vec<(usize, usize)> = vec![];

    for line in read_to_string(input).unwrap().lines() {
        let line_bytes = line.as_bytes();

        let first_digit = line_bytes.iter().enumerate().find_map(|(bindex, _)| find_from_bytes(line_bytes, bindex));
        let last_digit = line_bytes.iter().enumerate().rev().find_map(|(bindex, _)| find_from_bytes(line_bytes, bindex));

        data.push((
            first_digit.unwrap(),
            last_digit.unwrap(),
        ))
    }

    return calibrate(data);
}

fn find_from_bytes(line_bytes: &[u8], bindex: usize) -> Option<usize> {
    let byte_numbers: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    if line_bytes[bindex].is_ascii_digit() {
        return Some((line_bytes[bindex] - b'0') as usize);
    } else {
        return byte_numbers
            .iter()
            .enumerate()
            .find(|(_, name)| line_bytes[bindex..].starts_with(name))
            .map(|(num, _)| num + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_result_for_part_1() {
        let input = "input_test_part_1";

        assert_eq!(first_part(input), 142);
    }

    #[test]
    fn it_returns_the_result_for_part_2() {
        let input = "input_test_part_2";

        assert_eq!(second_part(input), 281);
    }
}
