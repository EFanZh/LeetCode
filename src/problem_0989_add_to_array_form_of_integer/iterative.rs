pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num = num;
        let mut k = k as u32;
        let mut digits = num.iter_mut().rev();

        while k != 0 {
            if let Some(target) = digits.next() {
                *target += (k % 10) as i32;
                k /= 10;

                if let Some(normalized_target) = (*target as u32).checked_sub(10) {
                    *target = normalized_target as _;
                    k += 1;
                }
            } else {
                let mut buffer = [0; 10];
                let mut i = 9;

                loop {
                    buffer[i] = (k % 10) as i32;
                    k /= 10;

                    if k == 0 {
                        break;
                    }

                    i -= 1;
                }

                num.splice(0..0, buffer[i..].iter().copied());

                break;
            }
        }

        num
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        Self::add_to_array_form(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
