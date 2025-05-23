pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_tag_name(input: &str) -> Option<(&str, &str)> {
        let search_area = if input.len() <= 9 { input } else { &input[..9] };

        let length = search_area
            .bytes()
            .position(|c| !c.is_ascii_uppercase())
            .unwrap_or(search_area.len());

        if length == 0 {
            None
        } else {
            Some(input.split_at(length))
        }
    }

    fn parse_cdata(input: &str) -> Option<&str> {
        let input = input.strip_prefix("<![CDATA[")?;
        let length = input.find("]]>")?;

        Some(&input[length..])
    }

    fn parse_text(input: &str) -> Option<&str> {
        let length = input.bytes().position(|c| c == b'<').unwrap_or(input.len());

        if length == 0 { None } else { Some(&input[length..]) }
    }

    fn parse_content(mut input: &str) -> &str {
        while let Some(next_input) = Self::parse_tag(input)
            .or_else(|| Self::parse_cdata(input))
            .or_else(|| Self::parse_text(input))
        {
            input = next_input;
        }

        input
    }

    fn parse_tag(input: &str) -> Option<&str> {
        let input = input.strip_prefix('<')?;
        let (tag_name, input) = Self::parse_tag_name(input)?;
        let input = input.strip_prefix('>')?;
        let input = Self::parse_content(input);
        let input = input.strip_prefix("</")?;
        let input = input.strip_prefix(tag_name)?;

        input.strip_prefix('>')
    }

    pub fn is_valid(code: String) -> bool {
        Self::parse_tag(&code) == Some("")
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_valid(code: String) -> bool {
        Self::is_valid(code)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
