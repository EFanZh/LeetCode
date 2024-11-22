pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let [mut a, mut b, mut c] = <[_; 3]>::map(amount.try_into().ok().unwrap(), |x| x as u16);

        if b < a {
            mem::swap(&mut a, &mut b);
        }

        if c < b {
            mem::swap(&mut b, &mut c);
        }

        if b < a {
            mem::swap(&mut a, &mut b);
        }

        i32::from(if a + b < c { c } else { (a + b + c + 1) / 2 })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn fill_cups(amount: Vec<i32>) -> i32 {
        Self::fill_cups(amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
