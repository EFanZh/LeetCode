pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Group accounts by user name.

        let mut user_names: HashMap<&str, Vec<&[String]>> = HashMap::new();

        for user in &accounts {
            let (name, emails) = user.split_first().unwrap();

            user_names
                .entry(name.as_str())
                .and_modify(|accounts| accounts.push(emails))
                .or_insert_with(|| vec![emails]);
        }

        // Get connected components in each group.

        let mut result = Vec::new();
        let mut graph = HashMap::<&str, Vec<&str>>::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        for (name, accounts) in user_names {
            // Build connection graph.

            for emails in accounts {
                let (first_email, rest) = emails.split_first().unwrap();

                graph.entry(first_email).or_insert_with(Vec::new);

                for email in rest {
                    graph
                        .entry(first_email)
                        .and_modify(|nexts| nexts.push(email))
                        .or_insert_with(|| vec![email]);

                    graph
                        .entry(email)
                        .and_modify(|nexts| nexts.push(first_email))
                        .or_insert_with(|| vec![first_email]);
                }
            }

            // Find connected components.

            for mut node in graph.keys().copied() {
                if visited.insert(node) {
                    let mut account = vec![name.to_string()];

                    loop {
                        account.push(node.to_string());

                        for &next in &graph[node] {
                            if visited.insert(next) {
                                stack.push(next);
                            }
                        }

                        if let Some(new_node) = stack.pop() {
                            node = new_node;
                        } else {
                            break;
                        }
                    }

                    account[1..].sort_unstable();

                    result.push(account);
                }
            }

            visited.clear();
            graph.clear();
        }

        result
    }
}

impl super::Solution for Solution {
    fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        Self::accounts_merge(accounts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
