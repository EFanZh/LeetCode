pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_ip_v4_component(component: &[u8]) -> bool {
        matches!(
            component,
            [b'0'..=b'9']
                | [b'1'..=b'9', b'0'..=b'9']
                | [b'1', b'0'..=b'9', b'0'..=b'9']
                | [b'2', b'0'..=b'4', b'0'..=b'9']
                | [b'2', b'5', b'0'..=b'5']
        )
    }

    fn is_ip_v4(ip: &[u8]) -> bool {
        let mut iter = ip.split(|&c| c == b'.');

        iter.next().map_or(false, Self::is_ip_v4_component)
            && iter.next().map_or(false, Self::is_ip_v4_component)
            && iter.next().map_or(false, Self::is_ip_v4_component)
            && iter.next().map_or(false, Self::is_ip_v4_component)
            && iter.next().is_none()
    }

    fn is_ip_v6_component(component: &[u8]) -> bool {
        (1..=4).contains(&component.len()) && component.iter().all(u8::is_ascii_hexdigit)
    }

    fn is_ip_v6(ip: &[u8]) -> bool {
        let mut iter = ip.split(|&c| c == b':');

        iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().map_or(false, Self::is_ip_v6_component)
            && iter.next().is_none()
    }

    pub fn valid_ip_address(ip: String) -> String {
        let result = if Self::is_ip_v4(ip.as_bytes()) {
            "IPv4"
        } else if Self::is_ip_v6(ip.as_bytes()) {
            "IPv6"
        } else {
            "Neither"
        };

        result.to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_ip_address(ip: String) -> String {
        Self::valid_ip_address(ip)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
