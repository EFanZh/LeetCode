pub mod look_ahead;
pub mod state_machine;
pub mod state_machine_2;

pub trait Solution {
    fn entity_parser(text: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                "&amp; is an HTML entity but &ambassador; is not.",
                "& is an HTML entity but &ambassador; is not.",
            ),
            ("and I quote: &quot;...&quot;", "and I quote: \"...\""),
            (" &frasl; &quot; &apos; ZooP)x:6~", " / \" ' ZooP)x:6~"),
            ("&&gt;", "&>"),
            ("&&&", "&&&"),
            (
                "x &gt; y &amp;&amp; x &lt; y is always false",
                "x > y && x < y is always false",
            ),
            ("&&&;", "&&&;"),
            ("&a", "&a"),
            ("&a&", "&a&"),
            ("&ax", "&ax"),
            ("&am", "&am"),
            ("&am&", "&am&"),
            ("&amp", "&amp"),
            ("&&&amp&&", "&&&amp&&"),
            ("&ampx", "&ampx"),
            ("&ap", "&ap"),
            ("&ap&", "&ap&"),
            ("&apx", "&apx"),
            ("&apo", "&apo"),
            ("&apo&", "&apo&"),
            ("&apox", "&apox"),
            ("&apos", "&apos"),
            ("&apos&", "&apos&"),
            ("&aposx", "&aposx"),
            ("&f", "&f"),
            ("&f&", "&f&"),
            ("&fx", "&fx"),
            ("&fr", "&fr"),
            ("&fr&", "&fr&"),
            ("&frx", "&frx"),
            ("&fra", "&fra"),
            ("&fra&", "&fra&"),
            ("&frax", "&frax"),
            ("&fras", "&fras"),
            ("&fras&", "&fras&"),
            ("&frasx", "&frasx"),
            ("&frasl", "&frasl"),
            ("&frasl&", "&frasl&"),
            ("&fraslx", "&fraslx"),
            ("&g", "&g"),
            ("&g&", "&g&"),
            ("&gx", "&gx"),
            ("&gt", "&gt"),
            ("&gt&", "&gt&"),
            ("&gtx", "&gtx"),
            ("&l", "&l"),
            ("&l&", "&l&"),
            ("&lx", "&lx"),
            ("&lt", "&lt"),
            ("&lt&", "&lt&"),
            ("&ltx", "&ltx"),
            ("&q", "&q"),
            ("&q&", "&q&"),
            ("&qx", "&qx"),
            ("&qu", "&qu"),
            ("&qu&", "&qu&"),
            ("&qux", "&qux"),
            ("&quo", "&quo"),
            ("&quo&", "&quo&"),
            ("&quox", "&quox"),
            ("&quot", "&quot"),
            ("&quot&", "&quot&"),
            ("&quotx", "&quotx"),
        ];

        for (text, expected) in test_cases {
            assert_eq!(S::entity_parser(text.to_string()), expected);
        }
    }
}
