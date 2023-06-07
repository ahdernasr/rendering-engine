/*
Need to try and support:

Comments
Doctype declarations
Escaped characters (like &amp;) and CDATA sections
Self-closing tags: <br/> or <br> with no closing tag
Error handling (e.g. unbalanced or improperly nested tags)
Namespaces and other XHTML syntax: <html:body>
Character encoding detection

 */
use rustc_lexer::is_whitespace;
use std::collections::HashMap;
mod domtree;

struct Parser {
    pos: usize, //index of the next unprocessed character
    input: String,
}

impl Parser {
    //Read the current character
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
    //Check if the characters start with a string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
    //Check if the all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    //Consume a character; return the current character and move pos to the next
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    //Consume a character while a certain test is true and all text is not consumed
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    //uses consume_while function to consume whitespace (discards it)
    fn consume_whitespace(&mut self) {
        self.consume_while(is_whitespace);
    }

    //Consumes a tag name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '!' | '-' => true,
            _ => false,
        })
    }

    //Consumes an parses a node (different parsing based on node type)
    fn parse_node(&mut self) -> domtree::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    //Consumes an parses a piece of text (text element)
    fn parse_text(&mut self) -> domtree::Node {
        domtree::text(self.consume_while(|c| c != '<'))
    }

    //Consumes and parses an element
    //TODO, the assert function is bad error handling,
    fn parse_element(&mut self) -> domtree::Node {
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        //Element is a comment, handle the comment tag
        if tag_name == "!--" {
            let comment_data = self.parse_comment();
            return domtree::comment(comment_data);
        }
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return domtree::elem(tag_name, attrs, children);
    }

    /*

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    */
    fn parse_comment(&mut self) -> String {
        let mut result = String::new();

        while !self.eof() {
            if self.next_char() == '-' {
                self.consume_char();
                if self.next_char() == '-' {
                    self.consume_char();
                    if self.next_char() == '>' {
                        self.consume_char();
                        //end of comment reached, return result
                        return result;
                    } else {
                        //false end of comment, add falsely consumed --
                        result.push_str("--");
                    }
                } else {
                    //false end of comment, add falsely consumed -
                    result.push('-');
                }
            } else {
                result.push(self.consume_char());
            }
        }
        return result;
    }

    //Consumes an attribute
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    //Consumes an attribute value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    //Consumes multiple attributes and then parses the attributes individually
    fn parse_attributes(&mut self) -> domtree::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    //Consumes multiple nodes then parses them individually
    fn parse_nodes(&mut self) -> Vec<domtree::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

//Main parsing function
pub fn parse(source: String) -> domtree::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        domtree::elem("html".to_string(), HashMap::new(), nodes)
    }
}

/*
   //Consumes an parses a piece of text (text element)
   fn parse_text(&mut self) -> domtree::Node {
       if self.next_char() == '/' {
           //possible comment detected, will parse comment
           self.consume_char();
           if self.next_char() == '/' {
               self.consume_char(); //removes the second back_slash
               self.parse_comment()
           } else {
               //text detected, returns incorrectly parsed "/" and add it to rest of the text and parses it
               let mut text = "/".to_owned();
               let text_rest = self.consume_while(|c| c != '<');
               text.push_str(&text_rest);
               domtree::text(text)
           }
       } else {
           //parses text normally
           domtree::text(self.consume_while(|c| c != '<'))
       }
   }

   fn parse_comment(&mut self) -> domtree::Node {
       domtree::comment(self.consume_while(|c| c != '<'))
   }


*/
