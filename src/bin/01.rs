advent_of_code::solution!(1);

fn extract_direction(input: &str) -> char {
    input.chars().nth(0).expect("No chars in input")
}
fn extract_distance(input: &str) -> i32 {
    input[1..].parse::<i32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {

    // Part 1: Count how many times the dial lands exactly on 0
    let mut zero_count: i32 = 0;
    let mut dial_position: i8 = 50;
    let mut end_position:i8 = 0;

    for rotation in input.split_whitespace() {
        let direction: char = extract_direction(rotation);
        let distance: i32 = extract_distance(rotation);

        let dial_movement = if direction == 'L' {
            -distance
        } else {
            distance
        };

        // End Position
        end_position = ((((dial_position as i32 + dial_movement) % 100) + 100) % 100) as i8;

        // Lands on 0
        let lands_on_zero = (end_position == 0) as i32;
        if lands_on_zero == 1 {
            zero_count += 1;
        }

        dial_position = end_position;

    }

    Option::from(zero_count as u64)

}

pub fn part_two(input: &str) -> Option<u64> {

    // Part 1: Count how many times the dial lands exactly on 0
    let mut dial_position: i8 = 50;
    let mut end_position:i8 = 0;
    let mut total_crossings: i32 = 0;

    for rotation in input.split_whitespace() {
        let direction: char = extract_direction(rotation);
        let distance: i32 = extract_distance(rotation);

        let dial_movement = if direction == 'L' {
            -distance
        } else {
            distance
        };

        // End Position
        end_position = ((((dial_position as i32 + dial_movement) % 100) + 100) % 100) as i8;

        // Sign of movement tells us which direction
        // Comparison tells us if we wrapped
        let direction_sign = dial_movement.signum();
        let wrapped = ((end_position as i32 - dial_position as i32) * direction_sign) < 0;
        let crosses_zero = wrapped as i32;

        // Full Wraps
        let full_wraps = distance.abs() / 100;

        // Lands on 0
        let lands_on_zero = (end_position == 0) as i32;

        // Total Crossings
        total_crossings += full_wraps + crosses_zero + lands_on_zero;

        // Reset current dial position
        dial_position = end_position;
    }

    Some(total_crossings as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
