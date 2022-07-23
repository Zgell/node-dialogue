use std::collections::HashMap;

pub trait Node {
    fn print(&self) -> u32;  // Executes node event and returns ID of next node
    fn connect(&mut self, id: u32); // Sets next node to the given ID
}

struct DNode {
    next_node: u32,
    text: String,
}
impl Node for DNode {
    fn print(&self) -> u32 {
        println!("{}", self.text);
        self.next_node
    }
    fn connect(&mut self, id: u32) {
        self.next_node = id;
    }
}

struct Dialogue {
    nodes: HashMap<u32, Box<dyn Node>>,
}
impl Dialogue {
    fn talk(&mut self) {
        let mut next_node = 1;
        while next_node != 0 {
            let current_node = self.nodes.get_mut(&next_node);
            match current_node {
                Some(node) => {
                    next_node = node.print();
                },
                None => {
                    next_node = 0;
                }
            }
        }
    }
    fn insert_node(&mut self, key: u32, value: Box<dyn Node>) {
        /*
        Inserts a node into the structure.
        Equivalent to "Dialogue.nodes.insert(key, value)".
        */
        self.nodes.insert(key, value);
    }
    fn connect_nodes(&mut self, from_id: u32, to_id: u32) {
        let node_wrapped = self.nodes.get_mut(&from_id);
        match node_wrapped {
            Some(node) => {
                node.connect(to_id);
            },
            None => {},
        }
    }
}


fn main() {
    let mut conversation = Dialogue {
        nodes: HashMap::new(),
    };

    let node_1 = DNode {
        next_node: 0,
        text: String::from("First dialogue line!"),
    };
    conversation.insert_node(1, Box::new(node_1));

    let node_2 = DNode {
        next_node: 0,
        text: String::from("This is the second dialogue line, terminated by a newline.")
    };
    conversation.insert_node(2, Box::new(node_2));
    conversation.connect_nodes(1, 2);

    let node_3 = DNode {
        next_node: 0,
        text: String::from("This is the third dialogue line! Very cool!")
    };
    conversation.insert_node(3, Box::new(node_3));
    conversation.connect_nodes(2, 3);

    // Print out the dialogue
    conversation.talk()
}
