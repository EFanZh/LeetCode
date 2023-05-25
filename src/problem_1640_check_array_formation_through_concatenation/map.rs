pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        const EMPTY: Vec<i32> = Vec::new();

        let mut pieces_by_first = [EMPTY; 100];

        for piece in pieces {
            let first = piece[0] as u32 as usize;

            pieces_by_first[first - 1] = piece;
        }

        let mut i = 0;

        while let Some([first, lhs @ ..]) = arr.get(i..) {
            if let Some(rhs) = pieces_by_first[*first as u32 as usize - 1].get(1..) {
                if lhs.starts_with(rhs) {
                    i += rhs.len() + 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        Self::can_form_array(arr, pieces)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
