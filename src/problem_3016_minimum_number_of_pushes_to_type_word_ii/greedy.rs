pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = [0_u32; 26];

        for c in word.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        drop(word);

        counts.sort_unstable();

        let mut result = 0;
        let mut assignment = 8;

        for count in counts.iter().rev().copied() {
            if count == 0 {
                break;
            }

            result += (assignment / 8) * count;
            assignment += 1;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_pushes(word: String) -> i32 {
        Self::minimum_pushes(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
