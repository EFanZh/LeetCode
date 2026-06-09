pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        let (first, rest) = nums.split_first_chunk::<3>().unwrap();
        let [mut x, mut y, mut z] = *first;

        (x, y, z) = if y < x {
            if z < x {
                if z < y { (z, y, x) } else { (y, z, x) }
            } else {
                (y, x, z)
            }
        } else {
            if z < y {
                if z < x { (z, x, y) } else { (x, z, y) }
            } else {
                (x, y, z)
            }
        };

        for &num in rest {
            if num < x {
                x = num;
            } else if num > y {
                (y, z) = if num > z { (z, num) } else { (num, z) };
            }
        }

        y + z - x
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
        Self::maximize_expression_of_three(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
