pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        const POWERS_OF_TEN: [i32; 9] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];
        const BOUNDARIES: [i32; 9] = [1, 10, 190, 2_890, 38_890, 488_890, 5_888_890, 68_888_890, 788_888_890];

        let (number_length, total_digits) = match BOUNDARIES.binary_search(&n) {
            Ok(i) => (i as i32 + 1, BOUNDARIES[i]),
            Err(i) => (i as i32, BOUNDARIES[i - 1]),
        };

        let remaining_digits = n - total_digits;
        let number = POWERS_OF_TEN[(number_length - 1) as usize] + remaining_digits / number_length;
        let digit_index = remaining_digits % number_length;

        (number / POWERS_OF_TEN[(number_length - 1 - digit_index) as usize]) % 10
    }
}

impl super::Solution for Solution {
    fn find_nth_digit(n: i32) -> i32 {
        Self::find_nth_digit(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
