pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let k = k as usize;
        let max_first = (n - k + 1) as i32;
        let last_index = k - 1;
        let mut result = Vec::new();

        let mut buffer = {
            let mut i = 0;

            [(); 20].map(|()| {
                i += 1;
                i
            })
        };

        let buffer = &mut buffer[..k];
        let mut i = last_index;

        loop {
            result.push(buffer.to_vec());

            loop {
                if let Some(value) = buffer.get_mut(i) {
                    if *value == max_first + i as i32 {
                        i = i.wrapping_sub(1);
                    } else {
                        *value += 1;

                        let mut prev = *value;

                        for target in &mut buffer[i + 1..] {
                            prev += 1;
                            *target = prev;
                        }

                        i = last_index;

                        break;
                    }
                } else {
                    return result;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::combine(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
