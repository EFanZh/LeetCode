pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let i = seats.iter().position(|&x| x != 0).unwrap();
        let mut iter = seats[i + 1..].iter().map(|&x| x != 0);
        let mut result = i as i32;

        loop {
            loop {
                if let Some(value) = iter.next() {
                    if !value {
                        break;
                    }
                } else {
                    return result;
                }
            }

            let mut length = 1;

            loop {
                if let Some(value) = iter.next() {
                    if value {
                        result = result.max((length + 1) / 2);

                        break;
                    }

                    length += 1;
                } else {
                    return result.max(length);
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        Self::max_dist_to_closest(seats)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
