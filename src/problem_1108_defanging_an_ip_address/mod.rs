pub mod brute_force;

pub trait Solution {
    fn defang_i_paddr(address: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1.1.1.1", "1[.]1[.]1[.]1"), ("255.100.50.0", "255[.]100[.]50[.]0")];

        for (address, expected) in test_cases {
            assert_eq!(S::defang_i_paddr(address.to_string()), expected);
        }
    }
}
