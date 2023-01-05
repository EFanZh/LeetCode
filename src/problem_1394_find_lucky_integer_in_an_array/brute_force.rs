pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counts = [0_u16; 500];

        for &value in &arr {
            counts[value as u32 as usize - 1] += 1;
        }

        arr.into_iter()
            .filter(|&i| {
                let i = i as u32;

                u32::from(counts[i as usize - 1]) == i
            })
            .max()
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_lucky(arr: Vec<i32>) -> i32 {
        Self::find_lucky(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
