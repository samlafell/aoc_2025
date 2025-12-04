advent_of_code::solution!(3);

fn find_max_two_digit(digits: &[u32]) -> u32 {
    // Safety check (though your caller handles this, it's good practice)
    if digits.len() < 2 {
        return 0;
    }

    let mut max_joltage = 0;

    // Create a reverse iterator
    let mut iter = digits.iter().rev();

    // 1. Initialize the "right-side max" with the very last digit
    // We use unwrap() here because we checked len >= 2 above.
    let mut max_suffix = *iter.next().unwrap();

    // 2. Iterate backwards through the remaining digits (the "left" digits)
    for &left_digit in iter {
        // Calculate the potential joltage with this digit as the tens place
        let current_joltage = (left_digit * 10) + max_suffix;

        if current_joltage > max_joltage {
            max_joltage = current_joltage;
        }

        // Update the max_suffix if the current digit is bigger
        // than what we've seen to the right so far.
        if left_digit > max_suffix {
            max_suffix = left_digit;
        }
    }

    max_joltage
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;

    for line in input.lines() {
        // Convert ONCE to Vec<u32>
        let digits: Vec<u32> = line.trim()
            .chars()
            .filter_map(|c| c.to_digit(10))  // ← Safely convert, skip non-digits
            .collect();

        if digits.len() < 2 { continue; }  // Need at least 2 digits!

        // Now work with the Vec<u32>
        let max_for_line = find_max_two_digit(&digits);
        total_joltage += max_for_line as u64;
    }

    Some(total_joltage)
}


pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(98+89+78+92));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
