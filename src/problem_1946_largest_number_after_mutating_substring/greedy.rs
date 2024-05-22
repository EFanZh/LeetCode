pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut num = num;

        if change != [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            let change = <[_; 10]>::map(change.try_into().ok().unwrap(), |x| b'0' + x as u8);
            let mut num_bytes = num.into_bytes();
            let mut iter = num_bytes.iter_mut();

            'outer: while let Some(mut current) = iter.next() {
                let mut target = change[usize::from(*current) - usize::from(b'0')];

                if *current < target {
                    loop {
                        *current = target;

                        if let Some(c) = iter.next() {
                            current = c;
                            target = change[usize::from(*current) - usize::from(b'0')];

                            if *current <= target {
                                continue;
                            }
                        }

                        break 'outer;
                    }
                }
            }

            num = String::from_utf8(num_bytes).ok().unwrap();
        }

        num
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_number(num: String, change: Vec<i32>) -> String {
        Self::maximum_number(num, change)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
