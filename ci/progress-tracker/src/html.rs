fn write_attribute(attribute: &(&str, &str), target: &mut String) {
    target.push_str(attribute.0.as_ref());
    target.push_str("=\"");

    for c in attribute.1.chars() {
        match c {
            '<' => target.push_str("&lt;"),
            '>' => target.push_str("&gt;"),
            '"' => target.push_str("&quot;"),
            '\'' => target.push_str("&apos;"),
            '&' => target.push_str("&amp;"),
            _ => target.push(c),
        }
    }

    target.push('\"');
}

pub struct Writer<'a> {
    buffer: &'a mut String,
}

impl<'a> Writer<'a> {
    pub fn on(buffer: &'a mut String) -> Self {
        Self { buffer }
    }

    fn write_attributes(&mut self, attributes: &[(&str, &str)]) {
        for attribute in attributes.iter() {
            self.buffer.push(' ');
            write_attribute(attribute, self.buffer);
        }
    }

    pub fn element<B: FnMut(&mut Self)>(&mut self, tag: &str, attributes: &[(&str, &str)], mut body: B) {
        self.buffer.push('<');
        self.buffer.push_str(tag);

        self.write_attributes(attributes);

        self.buffer.push('>');

        body(self);

        self.buffer.push_str("</");
        self.buffer.push_str(tag);
        self.buffer.push('>');
    }

    pub fn empty_element(&mut self, tag: &str, attributes: &[(&str, &str)]) {
        self.buffer.push('<');
        self.buffer.push_str(tag);

        self.write_attributes(attributes);

        self.buffer.push_str(" />");
    }

    pub fn text(&mut self, text: &str) {
        for c in text.chars() {
            match c {
                '<' => self.buffer.push_str("&lt;"),
                '>' => self.buffer.push_str("&gt;"),
                '&' => self.buffer.push_str("&amp;"),
                _ => self.buffer.push(c),
            }
        }
    }

    pub fn raw(&mut self, text: &str) {
        self.buffer.push_str(text);
    }
}
