
use crate::dom;


pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {input: source, pos: 0}.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string() , dom::AttrMap::new(), nodes)
    }
}

struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_string(&mut self, string: &str) -> Result<(),()> {
        if !self.starts_with(string) {
            return Err(());
        }

        let mut len = string.chars().count();

        while len > 0 {
            self.consume_char();
            len -= 1;
        }

        Ok(())
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.eof() && test(self.next_char()) {
                result.push(self.consume_char());
            }

            result
    }

    fn consume_till_str(&mut self, string: &str) -> String {
        let mut result = String::new();
        while !self.eof() && !self.starts_with(string) {
            result.push(self.consume_char());
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // fn consume_str_if_starts_with(&mut self, string: &str) {
    //     if self.starts_with(string) {
    //         self.consume_string(string).unwrap();
    //     }
    // }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A' ..= 'Z' | '0' ..= '9' => true,
            _ => false
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => {
                self.consume_char();
                if self.starts_with("!--") {
                    self.parse_comment()
                } else {
                    self.parse_element()
                }
            },
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // assert_eq!('<',self.consume_char());
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        // assert_eq!('>', self.consume_char());
        if let Ok(_) = self.consume_string("/>") {
            return dom::elem(tag_name, attrs, vec![]);
        }

        self.consume_char(); //consuming >

        let children = self.parse_nodes();

        // assert_eq!(self.consume_char(),'<');
        // assert_eq!(self.consume_char(), '/');
        // if self.starts_with("</"){
        //     assert_eq!(self.parse_name(), tag_name);
        //     assert_eq!(self.consume_char(), '>');
        // }
        let closing_tag = format!("</{}>", tag_name);
        let _ = self.consume_string(&closing_tag);

        dom::elem(tag_name, attrs, children)
    }

    fn parse_comment(&mut self) -> dom::Node {
        // assert_eq!(self.consume_char(), '!');
        // assert_eq!(self.consume_char(), '-');
        // assert_eq!(self.consume_char(), '-');
        self.consume_string("!--").expect("Some weird error occurred");

        let comment = dom::comment(self.consume_till_str("-->"));
        // assert_eq!(self.consume_char(), '-');
        // assert_eq!(self.consume_char(), '-');
        // assert_eq!(self.consume_char(), '>');

        let _ = self.consume_string("-->"); //consumes string if it exists

        comment
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.consume_whitespace();
        if self.next_char() == '=' {
            self.consume_char();
            let value = self.parse_attr_value();
            return (name, value);
        }

        (name, "true".to_string())
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.next_char();
        let hasquote = open_quote == '\''|| open_quote == '"';
        if hasquote {
            self.consume_char();
        }
        let value = self.consume_while(|c| (hasquote && c != open_quote)||(!hasquote && !c.is_whitespace()&&c!='>'));
        // assert_eq!(open_quote, self.consume_char());
        if hasquote && self.next_char() == open_quote {
            self.consume_char();
        }
        value
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = dom::AttrMap::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '>' || self.starts_with("/>") {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }

        attributes
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

}