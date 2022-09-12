pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut result = String::new();
        let mut prev = 0;

        for c in target.bytes() {
            let c = c - b'a';
            let prev_row = usize::from(prev / 5);
            let prev_column = usize::from(prev % 5);
            let row = usize::from(c / 5);
            let column = usize::from(c % 5);

            if prev_row < row {
                if prev_column < column {
                    result.extend(iter::repeat('R').take(column - prev_column));
                } else {
                    result.extend(iter::repeat('L').take(prev_column - column));
                }

                result.extend(iter::repeat('D').take(row - prev_row));
            } else {
                result.extend(iter::repeat('U').take(prev_row - row));

                if prev_column < column {
                    result.extend(iter::repeat('R').take(column - prev_column));
                } else {
                    result.extend(iter::repeat('L').take(prev_column - column));
                }
            }

            result.push('!');

            prev = c;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn alphabet_board_path(target: String) -> String {
        Self::alphabet_board_path(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
