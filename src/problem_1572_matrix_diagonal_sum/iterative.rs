pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut result = 0;

        for (i, row) in mat.iter().map(Vec::as_slice).enumerate() {
            let j = n - 1 - i;

            result += row[i];

            if i != j {
                result += row[j];
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        Self::diagonal_sum(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
