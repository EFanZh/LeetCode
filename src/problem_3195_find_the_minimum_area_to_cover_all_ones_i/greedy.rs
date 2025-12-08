pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert;

impl Solution {
    fn get_range(mut iter: impl DoubleEndedIterator<Item = bool> + ExactSizeIterator) -> u32 {
        iter.rposition(convert::identity).map_or(0, |right| {
            iter.position(convert::identity)
                .map_or(1, |left| right as u32 - left as u32 + 1)
        })
    }

    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let h_size = Self::get_range((0..grid.first().map_or(0, Vec::len)).map(|x| grid.iter().any(|row| row[x] != 0)));
        let v_size = Self::get_range(grid.into_iter().map(|row| row.iter().any(|&value| value != 0)));

        (h_size * v_size).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        Self::minimum_area(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
