pub struct Solution;

impl Solution {
    fn next(mut n: i32) -> i32 {
        let mut next = 0;

        while n != 0 {
            let digit = n % 10;

            next += digit * digit;
            n /= 10;
        }

        next
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = Self::next(n);
        let mut fast = Self::next(slow);

        while slow != fast {
            slow = Self::next(slow);
            fast = Self::next(Self::next(fast));
        }

        slow == 1
    }
}

impl super::Solution for Solution {
    fn is_happy(n: i32) -> bool {
        Self::is_happy(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
