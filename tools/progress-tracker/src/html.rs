use std::fmt::{self, Display, Formatter};
use std::io::Write;

struct AttributeValue<'a>(&'a str);

impl Display for AttributeValue<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for c in self.0.chars() {
            match c {
                '<' => f.write_str("&lt;"),
                '>' => f.write_str("&gt;"),
                '"' => f.write_str("&quot;"),
                '\'' => f.write_str("&apos;"),
                '&' => f.write_str("&amp;"),
                _ => fmt::Write::write_char(f, c),
            }?;
        }

        Ok(())
    }
}

struct TextContent<'a>(&'a str);

impl Display for TextContent<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for c in self.0.chars() {
            match c {
                '<' => f.write_str("&lt;"),
                '>' => f.write_str("&gt;"),
                '&' => f.write_str("&amp;"),
                _ => fmt::Write::write_char(f, c),
            }?;
        }

        Ok(())
    }
}

pub struct ElementWriter<'a, 'b, W>
where
    W: Write + ?Sized,
{
    writer: &'a mut W,
    name: &'b str,
}

impl<'a, 'b, W> ElementWriter<'a, 'b, W>
where
    W: Write + ?Sized,
{
    pub fn new(writer: &'a mut W, name: &'b str) -> Self {
        Self::with_attributes(writer, name, <[(&'static str, &'static str); 0]>::default())
    }

    pub fn with_attributes(
        writer: &'a mut W,
        name: &'b str,
        attributes: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Self {
        write!(writer, "<{name}").unwrap();

        for (key, value) in attributes {
            write!(writer, " {}=\"{}\"", key.as_ref(), AttributeValue(value.as_ref())).unwrap();
        }

        write!(writer, ">").unwrap();

        Self { writer, name }
    }

    pub fn add_element<'c>(&mut self, name: &'c str) -> ElementWriter<'_, 'c, W> {
        ElementWriter::new(self.writer, name)
    }

    pub fn add_empty_element_with_attributes(
        &mut self,
        name: &str,
        attributes: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) {
        write!(self.writer, "<{name}").unwrap();

        for (key, value) in attributes {
            write!(self.writer, " {}=\"{}\"", key.as_ref(), AttributeValue(value.as_ref())).unwrap();
        }

        write!(self.writer, "/>").unwrap();
    }

    pub fn add_element_with_attributes<'c>(
        &mut self,
        name: &'c str,
        attributes: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> ElementWriter<'_, 'c, W> {
        ElementWriter::with_attributes(self.writer, name, attributes)
    }

    pub fn add_text(&mut self, text: &str) {
        write!(self.writer, "{}", TextContent(text)).unwrap();
    }

    pub fn add_raw(&mut self, content: &str) {
        write!(self.writer, "{content}").unwrap();
    }

    pub fn close(self) {
        drop(self);
    }
}

impl<W> Drop for ElementWriter<'_, '_, W>
where
    W: Write + ?Sized,
{
    fn drop(&mut self) {
        write!(self.writer, "</{}>", self.name).unwrap();
    }
}
