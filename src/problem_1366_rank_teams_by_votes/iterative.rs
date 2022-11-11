pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let mut counts = [[0_u16; 26]; 26];

        for vote in &votes {
            for (rank, c) in vote.bytes().enumerate() {
                counts[usize::from(c - b'A')][rank] += 1;
            }
        }

        let mut result = votes.into_iter().next().unwrap().into_bytes();

        result.sort_unstable_by(|&lhs, &rhs| {
            let lhs_counts = &counts[usize::from(lhs) - usize::from(b'A')];
            let rhs_counts = &counts[usize::from(rhs) - usize::from(b'A')];

            rhs_counts.cmp(lhs_counts).then_with(|| lhs.cmp(&rhs))
        });

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rank_teams(votes: Vec<String>) -> String {
        Self::rank_teams(votes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
