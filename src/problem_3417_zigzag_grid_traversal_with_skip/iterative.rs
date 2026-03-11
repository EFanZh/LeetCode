pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(grid.first().map_or(0, Vec::len) * grid.len());
        let (chunks, extra) = grid.as_chunks::<2>();

        for [forward, backward] in chunks {
            result.extend(forward.iter().step_by(2));
            result.extend(backward.iter().skip(1).step_by(2).rev());
        }

        for forward in extra {
            result.extend(forward.iter().step_by(2));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        Self::zigzag_traversal(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
