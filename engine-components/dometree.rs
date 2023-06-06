//create a dom tree programmatically?
//and then reverse this, go from a programmed dom tree to html

struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

enum NodeType {
    Text(String),
    Comment(String),
    Element(ElementData),

}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>

//function that constructs a text node given its data
fn text(data: String) -> Node {
    Node { 
        children: Vec::new(), 
        node_type: NodeType::Text(data)
    }
}

//function that constructs an element node given its name, attributes, and children
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) {
    Node {
        children: children, 
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}