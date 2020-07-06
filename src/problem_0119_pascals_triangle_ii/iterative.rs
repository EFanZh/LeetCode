pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let row_length = row_index + 1;
        let mut result = vec![1; row_length];
        let middle = row_length / 2;
        let mut prev = 1;

        for i in 1..middle {
            prev *= row_length - i;
            prev /= i;

            result[i] = prev as i32;
            result[row_index - i] = prev as i32;
        }

        if row_index > 1 && row_index % 2 == 0 {
            prev *= row_index - (middle - 1);
            prev /= middle;

            result[middle] = prev as i32;
        }

        result
    }
}

impl super::Solution for Solution {
    fn get_row(row_index: i32) -> Vec<i32> {
        Self::get_row(row_index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
