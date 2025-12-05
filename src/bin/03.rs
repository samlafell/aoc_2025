advent_of_code::solution!(3);

fn find_max_two_digit(digits: &[u32]) -> u32 {
    if digits.len() < 2 {
        return 0;
    }

    let mut max_joltage = 0;

    // Create a reverse iterator
    let mut iter = digits.iter().rev();

    // 1. Initialize the "right-side max" with the very last digit. We use unwrap() here because we checked len >= 2 above.
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


/// Finds the maximum 12-digit number by selecting exactly 12 digits from the input sequence.
///
/// This function solves a **subsequence selection problem**: given a sequence of digits,
/// select exactly 12 of them (maintaining their original order) to form the largest
/// possible 12-digit number.
///
/// # Algorithm: Dynamic Programming (DP)
///
/// We use a 2D DP table where `dp[position][count]` represents:
/// - **position**: How many digits from the input we've examined so far (0 to n)
/// - **count**: How many of those digits we've selected (0 to 12)
/// - **value**: The maximum number we can form with that position/count combination
///
/// # Example
///
/// Input: `987654321111111` (15 digits)
/// Goal: Select 12 digits to maximize the result
/// Answer: `987654321111` (we skip the last 3 ones)
///
/// ## How it works:
///
/// For input `987654321111111`:
/// ```text
/// Position 0 (digit 9):
///   dp[1][0] = 0        (examined 1 digit, selected 0)
///   dp[1][1] = 9        (examined 1 digit, selected 1: the 9)
///
/// Position 1 (digit 8):
///   dp[2][0] = 0        (examined 2 digits, selected 0)
///   dp[2][1] = 9        (examined 2, selected 1: just the 9, skip the 8)
///   dp[2][2] = 98       (examined 2, selected 2: both 9 and 8)
///
/// Position 2 (digit 7):
///   dp[3][3] = 987      (selected all three: 9, 8, 7)
///   ...and so on...
///
/// Final: dp[15][12] = 987654321111
/// ```
///
/// # State Transitions
///
/// At each position, we make a choice for the current digit:
/// 1. **Skip it**: `dp[i+1][j] = dp[i][j]` (don't select, carry forward previous best)
/// 2. **Select it**: `dp[i+1][j+1] = dp[i][j] * 10 + digit` (append digit to form larger number)
///
/// We take the maximum of all possible ways to reach each state.
///
/// # Returns
///
/// The maximum 12-digit number that can be formed, or 0 if input has fewer than 12 digits.
fn find_max_twelve_digit(digits: &[u32]) -> u64 {
    let n = digits.len();

    // Initialize the DP table
    // dp[pos][count] = the maximum number we can form by:
    //   - Examining the first `pos` digits of the input
    //   - Selecting exactly `count` of them
    //
    // Dimensions: (n+1) positions × 13 counts (0 through 12)
    // We need n+1 because position 0 means "haven't looked at any digits yet"
    let mut dp = vec![vec![0u64; 13]; n + 1];

    // Base case: dp[0][0] = 0
    // Before examining any digits, if we've selected 0 digits, the number formed is 0
    // All other dp[0][j] for j > 0 remain 0 (impossible to select digits without seeing any)

    // Process each digit from left to right
    // This is critical: we process in the natural reading order so that when we
    // "append" a digit (multiply by 10 and add), it goes in the correct position
    for i in 0..n {
        let digit = digits[i];

        // For each possible count of digits we could have selected so far
        // We can't have selected more than i+1 digits after examining i+1 positions
        // (we use i+1 because we're about to update position i+1 in the DP table)
        for j in 0..=12.min(i + 1) {
            // ═══════════════════════════════════════════════════════════════
            // DECISION 1: SKIP this digit
            // ═══════════════════════════════════════════════════════════════
            // If we skip the digit at position i, then after examining i+1 digits,
            // we've still only selected j digits (same as before).
            // The best number with j selections remains whatever it was at position i.
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);

            // ═══════════════════════════════════════════════════════════════
            // DECISION 2: SELECT this digit
            // ═══════════════════════════════════════════════════════════════
            // Only valid if we haven't already selected 12 digits (our target)
            if j < 12 {
                // If we select this digit:
                // - We had selected j digits before (value = dp[i][j])
                // - Now we're selecting one more (total = j+1 digits)
                // - The new number = old number × 10 + current digit
                //
                // Example: If dp[i][j] = 98 and digit = 7
                //          new_value = 98 × 10 + 7 = 987
                let new_value = dp[i][j] * 10 + digit as u64;

                // We might reach state (i+1, j+1) via multiple paths, so we
                // keep the maximum value among all paths
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(new_value);
            }
        }
    }

    // After examining all n digits, return the best value with exactly 12 selected
    // dp[n][12] = maximum 12-digit number formed by selecting 12 from all n digits
    dp[n][12]
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut total_joltage: u64 = 0;

    for line in input.lines() {
        // Convert ONCE to Vec<u32>
        let digits: Vec<u32> = line.trim()
            .chars()
            .filter_map(|c| c.to_digit(10))  // ← Safely convert, skip non-digits
            .collect();

        if digits.len() < 12 { continue; }  // Need at least 2 digits!

        // Now work with the Vec<u32>
        let max_for_line = find_max_twelve_digit(&digits);
        total_joltage += max_for_line as u64;
    }

    Some(total_joltage)
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
