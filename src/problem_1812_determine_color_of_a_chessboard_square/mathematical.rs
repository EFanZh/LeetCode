pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let [column, row]: [_; 2] = coordinates.as_bytes().try_into().ok().unwrap();

        (column ^ row) & 1 != 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn square_is_white(coordinates: String) -> bool {
        Self::square_is_white(coordinates)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
