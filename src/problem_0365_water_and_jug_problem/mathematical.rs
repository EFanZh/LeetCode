pub struct Solution;

impl Solution {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let next_y = x % y;

            x = y;
            y = next_y;
        }

        x
    }

    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        z == 0 || (z <= x + y && z % Self::gcd(x, y) == 0)
    }
}

impl super::Solution for Solution {
    fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        Self::can_measure_water(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
