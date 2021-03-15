pub mod standard;

pub trait Solution {
    fn find_lu_slength(a: String, b: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aba", "cdc"), 3),
            (("aaa", "bbb"), 3),
            (("aaa", "aaa"), -1),
            (("aefawfawfawfaw", "aefawfeawfwafwaef"), 17),
            (("horbxeemlgqpqbujbdagizcfairalg", "iwvtgyojrfhyzgyzeikqagpfjoaeen"), 30),
        ];

        for ((a, b), expected) in test_cases.iter().copied() {
            assert_eq!(S::find_lu_slength(a.to_string(), b.to_string()), expected);
        }
    }
}
