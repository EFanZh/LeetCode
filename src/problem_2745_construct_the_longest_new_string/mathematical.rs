pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        let x = x.cast_unsigned();
        let y = y.cast_unsigned();
        let z = z.cast_unsigned();

        let k = match x.cmp(&y) {
            Ordering::Less => x * 4 + 2,
            Ordering::Equal => x * 4,
            Ordering::Greater => y * 4 + 2,
        };

        (k + z * 2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        Self::longest_string(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
