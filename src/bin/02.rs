advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {


    let mut begin:i128 = 0;
    let mut end:i128 = 0;
    let mut count_var:i128 = 0;

    let binding = input.to_string();
    let slice: Vec<&str> = binding.split(",").collect();

    for range_string in slice {
        let parts: Vec<&str> = range_string.split("-").collect();
        begin = parts[0].trim().parse().unwrap();
        end = parts[1].trim().parse().unwrap();

        for int in begin..end+1 {
            let string_int = int.to_string();
            if string_int.len() % 2 == 0 {
                let midpoint = (string_int.len()) / 2;
                let first_half = &string_int[0..midpoint];
                let second_half = &string_int[midpoint..];
                if first_half == second_half {
                    count_var += int;
                }
            }
        }

        // Loop over beginning to end range.
        // for each number in the range, odd numbers dont do anything
        // even numbers cut them in

    }

    return Some(count_var as u64)

}


// Function for checking if we have a repeating pattern
pub fn is_repeating_pattern(s: &str, pattern_len: usize) -> bool {
    let mut chunks = s.as_bytes().chunks(pattern_len);
    let first_chunk = chunks.next().unwrap(); // or handle None?
    chunks.all(|chunk| chunk == first_chunk)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut final_number = 0_u64;

    let binding = input.to_string();
    let slice: Vec<&str> = binding
        .split(",")
        .filter(|s| !s.trim().is_empty())  // Skip empty strings!
        .collect();

    for range_string in slice {
        let parts: Vec<&str> = range_string.split("-").collect();

        // Skip if we don't have exactly 2 parts
        if parts.len() != 2 {
            continue;
        }

        // Try to parse, skip on error
        let range_start: u64 = match parts[0].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let range_end: u64 = match parts[1].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        for num in range_start..=range_end {
            let s = num.to_string();
            let len = s.len();

            // Try repetition counts from 2 to len
            for repetitions in 2..=len {
                if len % repetitions == 0 {
                    let pattern_len = len / repetitions;
                    if is_repeating_pattern(&s, pattern_len) {
                        final_number += num;
                        break;  // Found a valid repetition, count once!
                    }
                }
            }
        }
    }
    Some(final_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));  // ← Update this!
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
