pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let [target_a, target_b, target_c]: [_; 3] = target.try_into().ok().unwrap();
        let mut state = 0_u8;

        for triplet in triplets {
            let [a, b, c]: [_; 3] = triplet.try_into().ok().unwrap();

            if a <= target_a && b <= target_b && c <= target_c {
                state |= u8::from(a == target_a);
                state |= u8::from(b == target_b) << 1;
                state |= u8::from(c == target_c) << 2;

                if state == 0b_111 {
                    return true;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        Self::merge_triplets(triplets, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
