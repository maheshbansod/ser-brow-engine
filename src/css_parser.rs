
use std::str::FromStr;
use crate::css;

pub fn parse(source: String) -> css::Stylesheet {
    let style = Parser {input: source, pos: 0}.parse_style();

    style
}

impl css::Selector {
    pub fn specificity(&self) -> css::Specificity {
        let css::Selector::Simple(ref simple) = *self;

        let idc = simple.id.iter().count();
        let tagc = simple.tag_name.iter().count();
        let classc = simple.class.len();

        (idc, tagc, classc)
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

    //just consumes any string containing number or dot
    fn consume_number(&mut self) -> String {
        self.consume_while(|x| match x {
            '0'..='9'|'.' => true,
            _ => false
        })
    }

    fn valid_identifier_char(c: char) -> bool {
        match c {
            'a'..='z' | 'A' ..= 'Z' | '0' ..= '9' | '_' | '-' => true,
            _ => false,
        }
    }

    fn parse_identifier(&mut self) -> Option<String> {
        let id = self.consume_while(|c| match c {
            c if Parser::valid_identifier_char(c) => true,
            _ => false
        });

        // self.consume_whitespace();
        // if self.eof() {
        //     return None;
        // }
        // let c = self.next_char();

        // if id.len() == 0 || (c != '{' && c != ',' && c!= ':' && c!= ';') {
        //     None
        // } else {
            Some(id)
        // }
    }

    fn parse_simple_selector(&mut self) -> Result<css::SimpleSelector, String> {
        let mut selector = css::SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new()
        };

        while !self.eof() {
            self.consume_whitespace();
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = self.parse_identifier();
                    //write code to ignore set of ids w invalid identifier
                    // if selector.id.is_none() {
                    //     break;
                    // }
                },
                '.' => {
                    self.consume_char();
                    let class = self.parse_identifier();
                    //TODO: ignore if invalid class
                    let class = class.unwrap();
                    // println!("Parsed class '{}' :D", class);
                    selector.class.push(class);
                },
                '*' => {
                    self.consume_char();
                }
                c if Parser::valid_identifier_char(c) => {
                    let tag_name = self.parse_identifier();
                    //TODO: ignore invalidds
                    selector.tag_name = tag_name;
                },
                _ => break,
            }
        }

        return Ok(selector);
    }

    fn parse_rule(&mut self) -> css::Rule {
        css::Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<css::Selector> {
        let mut selectors = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            let c = self.next_char();
            // // println!("next selector starts with {}", c);

            match c {
                '{' => {
                    self.consume_char();
                    break;
                },
                ',' => {self.consume_char();},
                _ => {
                    let selector = self.parse_simple_selector();
                    if let Ok(selector) = selector {
                        let selector = css::Selector::Simple(selector);
                        selectors.push(selector);
                    } else{
                        self.consume_whitespace();
                        let vlen = selectors.len();
                        self.consume_while(|c| c != ','||(vlen == 0 && c!='}')||(vlen != 0 && c!='{')); //move towards next tag or end of selector list or end of content
                        if self.eof() {
                            break;
                        }
                        if self.next_char() != '{' {
                            self.consume_char();
                        }
                    }
                },
            }
        }

        selectors
    }

    fn parse_declarations(&mut self) -> Vec<css::Declaration> {
        let mut decs = vec![];

        loop {
            if self.eof() {
                break;
            }
            self.consume_whitespace();
            let c = self.next_char();
            match c {
                c if Parser::valid_identifier_char(c) => {
                    let declaration = self.parse_declaration();
                    if let Ok(declaration) = declaration {
                        decs.push(declaration);
                    }
                },
                ';' => {
                    self.consume_char();
                    self.consume_whitespace();
                    self.consume_string("}");
                },
                _ => {break;}
            }
        }

        decs
    }

    fn parse_declaration(&mut self) -> Result<css::Declaration, String> {
        let property = self.parse_identifier();
        if let Some(property) = property {
            // println!("parsing for {}", property);
            self.consume_whitespace();
            if self.eof() || self.next_char() != ':' {
                return Err("Expected ':' after name of property followed by the property value".to_string());
            }
            self.consume_char();//consume ':'
            self.consume_whitespace();
            let value = self.parse_value();
            
            if let Ok(value) = value {
                // println!("Found decratation: {}", property);
                return Ok(css::Declaration {
                    name: property,
                    value: value,
                })
            } else {
                // // println!("So the colour is not okay? {}", property);
                return Err(format!("Couldn't parse value for property {}",property));
            }
        } else {
            return Err("Couldnt parse property.".to_string());
        }
    }

    fn parse_value(&mut self) -> Result<css::Value, String> {
        let c = self.next_char();

        match c {
            '0'..='9' => {
                let val = f32::from_str(&self.consume_number()).unwrap(); //handle this too?
                let unit = if self.starts_with("px") {
                    self.consume_string("px");
                    css::Unit::Px
                } else {
                    css::Unit::None
                };
                if self.next_char() == ';'{
                    Ok(css::Value::Length(val, unit))
                } else {
                    Err("All declarations must be terminated by ;".to_string())
                }
            },
            _ => {
                if self.starts_with("rgb(") || self.starts_with("rgba("){
                    let val = self.parse_color();
                    if let Err(message) = val {
                        return Err(message.to_string());
                    }
                    let val = css::Value::ColorValue(val.unwrap());
                    if self.next_char() == ';'{
                        Ok(val)
                    } else {
                        // println!("Not terminated wth");
                        Err("All declarations must be terminated by ;".to_string())
                    }
                } else {
                    let val = self.parse_identifier().unwrap();
                    if val.len() > 0 && self.next_char() == ';'{
                        Ok(css::Value::Keyword(val))
                    } else {
                        Err("All declarations must be terminated by ;".to_string())
                    }
                }
            }
        }
    }

    fn parse_color(&mut self) -> Result<css::Color, &str>{
        self.consume_string("rgb");
        let hasalpha = self.next_char() == 'a';
        if hasalpha {
            self.consume_char(); //consume 'a'
        }
        self.consume_char(); //consume '('
        self.consume_whitespace();
        let number = u8::from_str(&self.consume_number());
        if let Err(_) = number {
            return Err("Incorrect format for color. Expected u8 values (u8,u8,u8[,u8])");
        }
        let r = number.unwrap();
        // println!("Red value {}!",r);
        self.consume_whitespace();
        if self.next_char() != ',' {
            return Err("Seperate the colour values with `,`");
        }
        self.consume_char();
        self.consume_whitespace();
        let number = u8::from_str(&self.consume_number());
        if let Err(_) = number {
            return Err("Incorrect format for color. Expected u8 values (u8,u8,u8[,u8])");
        }
        let g = number.unwrap();
        self.consume_whitespace();
        if self.next_char() != ',' {
            return Err("Seperate the colour values with `,`");
        }
        self.consume_char();
        self.consume_whitespace();
        let number = u8::from_str(&self.consume_number());
        if let Err(_) = number {
            return Err("Incorrect format for color. Expected u8 values (u8,u8,u8[,u8])");
        }
        let b = number.unwrap();
        // println!("Blue value {}!",b);
        let a = if hasalpha {
            self.consume_whitespace();
            if self.next_char() != ',' {
                return Err("Seperate the colour values with `,`");
            }
            self.consume_char();
            self.consume_whitespace();
            let number = u8::from_str(&self.consume_number());
            if let Err(_) = number {
                return Err("Incorrect format for color. Expected u8 values (u8,u8,u8[,u8])");
            }
            Ok(number.unwrap())
        } else {
            Ok(255)
        }?;
        self.consume_whitespace();
        if self.next_char() != ')' {
            return Err("no closing bracket found for the colour");
        }
        self.consume_char(); //consume )
        // println!("Alpha value {}!",a);

        Ok(css::Color{r,g,b,a})
    }

    fn parse_style(&mut self) -> css::Stylesheet {
        let mut rules = vec![];

        while !self.eof() {
            self.consume_whitespace();
            let ruru = self.parse_rule();
            // println!("Parsed a ruru!!\n{}", ruru);
            rules.push(ruru);
        }

        css::Stylesheet {rules}
    }
}