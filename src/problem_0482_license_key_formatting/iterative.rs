pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::same_item_push)]
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let letters = s.bytes().filter(|&x| x != b'-').count();

        let mut iter = s
            .bytes()
            .filter_map(|x| if x == b'-' { None } else { Some(x.to_ascii_uppercase()) });

        let (first_group_size, rest_groups) = if letters % k == 0 {
            (k, (letters / k).saturating_sub(1))
        } else {
            (letters % k, letters / k)
        };

        let mut result = Vec::with_capacity(letters + rest_groups);

        result.extend(iter.by_ref().take(first_group_size));

        for _ in 0..rest_groups {
            result.push(b'-');
            result.extend(iter.by_ref().take(k));
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn license_key_formatting(s: String, k: i32) -> String {
        Self::license_key_formatting(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
