struct StrSplit<'delimiter, 'document> {
    index: usize,
    delimiter: &'delimiter str,
    document: &'document str,
}

impl<'delimiter, 'document> Iterator for StrSplit<'delimiter, 'document> {
    type Item = &'document str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.document.len() {
            return None;
        }
        match self.document[self.index..].find(self.delimiter) {
            Some(pos) => {
                let result = &self.document[self.index..self.index + pos];
                self.index += pos + self.delimiter.len();
                Some(result)
            }
            None => {
                let result = &self.document[self.index..];
                self.index = self.document.len();
                Some(result)
            }
        }
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        index: 0,
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

fn main() {
    let document = "Here we have a huge document, that explains a lot of things.";
    let delim = ',';
    match str_before(document, delim) {
        Some(text) => println!("Text before {} = {}", delim, text),
        None => println!("There's no text before {}", delim),
    }
}
