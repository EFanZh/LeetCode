pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counts = HashMap::new();

        for cpdomain in &cpdomains {
            let split = cpdomain.find(' ').unwrap();
            let count = cpdomain[..split].parse::<u32>().unwrap();
            let mut domain = &cpdomain[split + 1..];

            loop {
                counts.entry(domain).and_modify(|c| *c += count).or_insert(count);

                if let Some(i) = domain.find('.') {
                    domain = &domain[i + 1..];
                } else {
                    break;
                }
            }
        }

        counts
            .into_iter()
            .map(|(domain, count)| format!("{} {}", count, domain))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        Self::subdomain_visits(cpdomains)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
