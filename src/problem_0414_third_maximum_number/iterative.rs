pub struct Solution;

use std::cmp::Ordering;
use std::mem;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter();
        let mut first = iter.next().unwrap();

        while let Some(mut second) = iter.next() {
            match second.cmp(&first) {
                Ordering::Less => {}
                Ordering::Equal => continue,
                Ordering::Greater => mem::swap(&mut first, &mut second),
            }

            while let Some(mut third) = iter.next() {
                match third.cmp(&second) {
                    Ordering::Less => {}
                    Ordering::Equal => continue,
                    Ordering::Greater => match third.cmp(&first) {
                        Ordering::Less => mem::swap(&mut second, &mut third),
                        Ordering::Equal => continue,
                        Ordering::Greater => third = mem::replace(&mut second, mem::replace(&mut first, third)),
                    },
                }

                for num in iter {
                    if num > third {
                        match num.cmp(&second) {
                            Ordering::Less => third = num,
                            Ordering::Equal => continue,
                            Ordering::Greater => match num.cmp(&first) {
                                Ordering::Less => third = mem::replace(&mut second, num),
                                Ordering::Equal => continue,
                                Ordering::Greater => third = mem::replace(&mut second, mem::replace(&mut first, num)),
                            },
                        }
                    }
                }

                return third;
            }

            break;
        }

        first
    }
}

impl super::Solution for Solution {
    fn third_max(nums: Vec<i32>) -> i32 {
        Self::third_max(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
