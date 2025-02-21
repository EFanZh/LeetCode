pub mod prefix_sums;

pub trait Solution {
    fn largest_variance(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aababbb", 3),
            ("abcde", 0),
            ("bbc", 1),
            ("aaaaabbba", 4),
            (
                "ykudzhiixwttnvtesiwnbcjmsydidttiyabbwzlfbmmycwjgzwhbtvtxyvkkjgfehaypiygpstkhakfasiloaveqzcywsiujvixcdnxpvvtobxgroznswwwipypwmdhldsoswrzyqthaqlbwragjrqwjxgmftjxqugoonxadazeoxalmccfeyqtmoxwbnphxih",
                12,
            ),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::largest_variance(s.to_string()), expected);
        }
    }
}
