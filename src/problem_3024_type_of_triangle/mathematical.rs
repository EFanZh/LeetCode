pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        const BUFFER: &str = "isoscelescalenequilateral";

        let isosceles = &BUFFER[..9];
        let scalene = &BUFFER[8..15];
        let equilateral = &BUFFER[14..];
        let [mut a, mut b, mut c] = <[_; 3]>::map(nums.try_into().ok().unwrap(), i32::cast_unsigned);

        'block: {
            let max = if b > a {
                if c > b { &mut c } else { &mut b }
            } else if c > a {
                &mut c
            } else {
                break 'block;
            };

            mem::swap(&mut a, max);
        }

        String::from(if b + c > a {
            if a == b {
                if a == c { equilateral } else { isosceles }
            } else if a == c || b == c {
                isosceles
            } else {
                scalene
            }
        } else {
            "none"
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn triangle_type(digits: Vec<i32>) -> String {
        Self::triangle_type(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
