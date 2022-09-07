pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut n = n as u32;
        let mut a = 0;
        let mut b = 0;
        let mut base = 1;

        loop {
            match n % 10 {
                0 => {
                    a += base;
                    b += base * 9;
                    n -= 10;
                }
                1 => {
                    if n == 1 {
                        b += base;

                        break;
                    }

                    a += base * 2;
                    b += base * 9;
                    n -= 10;
                }
                digit => {
                    a += base;
                    b += base * (digit - 1);
                }
            }

            n /= 10;

            if n == 0 {
                break;
            }

            base *= 10;
        }

        vec![a as _, b as _]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_no_zero_integers(n: i32) -> Vec<i32> {
        Self::get_no_zero_integers(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
