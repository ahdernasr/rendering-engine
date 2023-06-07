//create a dom tree programmatically?
//and then reverse this, go from a programmed dom tree to html
#![allow(dead_code)]
use std::collections::HashMap;


#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

//function that constructs a text node given its data
pub fn text(data: String) -> Node {
    Node { 
        children: Vec::new(), 
        node_type: NodeType::Text(data)
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(), 
        node_type: NodeType::Comment(data)
    }
}

//function that constructs an element node given its name, attributes, and children
pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children, 
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}