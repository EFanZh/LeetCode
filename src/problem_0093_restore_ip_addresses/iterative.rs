pub struct Solution {}

impl Solution {
    fn is_component(slice: &str) -> bool {
        if slice.starts_with('0') {
            slice.len() == 1
        } else {
            slice.parse::<u8>().is_ok()
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();

        for length_1 in 1.max(s.len().saturating_sub(9))..=3.min(s.len().saturating_sub(3)) {
            let (first, s) = s.split_at(length_1);

            if Self::is_component(first) {
                for length_2 in 1.max(s.len().saturating_sub(6))..=3.min(s.len().saturating_sub(2)) {
                    let (second, s) = s.split_at(length_2);

                    if Self::is_component(second) {
                        for length_3 in 1.max(s.len().saturating_sub(3))..=3.min(s.len().saturating_sub(1)) {
                            let (third, s) = s.split_at(length_3);

                            if Self::is_component(third) && Self::is_component(s) {
                                result.push(format!("{}.{}.{}.{}", first, second, third, s));
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn restore_ip_addresses(s: String) -> Vec<String> {
        Self::restore_ip_addresses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
