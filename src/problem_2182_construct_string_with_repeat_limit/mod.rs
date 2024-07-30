pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn repeat_limited_string(s: String, repeat_limit: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("cczazcc", 3), "zzcccac"),
            (("aababab", 2), "bbabaa"),
            (
                ("robnsdvpuxbapuqgopqvxdrchivlifeepy", 2),
                "yxxvvuvusrrqqppopponliihgfeeddcbba",
            ),
            (
                ("xyutfpopdynbadwtvmxiemmusevduloxwvpkjioizvanetecnuqbqqdtrwrkgt", 1),
                "zyxyxwxwvwvuvuvututstrtrtqpqpqponononmlmkmkjigifiededededcbaba",
            ),
        ];

        for ((s, repeat_limit), expected) in test_cases {
            assert_eq!(S::repeat_limited_string(s.to_string(), repeat_limit), expected);
        }
    }
}
