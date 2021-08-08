
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,

    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        if let Some(classes) = self.attributes.get("class") {
            classes.split(' ').collect()
        } else {
            HashSet::new()
        }
    }
}

pub fn text(data: String) -> Node {
    Node {children: vec![], node_type: NodeType::Text(data), }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

pub fn comment(comment: String) -> Node {
    Node {children: vec![], node_type: NodeType::Comment(comment)}
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            NodeType::Element(elem_data) => {
                write!(f, "<{}{}>\n{}\n</{}>",
                            elem_data.tag_name,
                            match elem_data.attributes.iter().map(|(key, val)| {
                                format!(" {}='{}'", key, val)
                            }).reduce(|a, b| {
                                a + &b
                            }) {
                                Some(text)=> text,
                                None => String::new()
                            },
                            self.children.iter()
                                .map(|x| format!("{}",x))
                                .collect::<Vec<String>>().join("\n"),
                            elem_data.tag_name
                        )

            },
            NodeType::Text(data) => {
                write!(f,"{}", data)
            },
            NodeType::Comment(comment) => {
                write!(f, "<!--{}-->", comment)
            }
        }
    }
}