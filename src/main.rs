use std::collections::HashMap;
use std::io;

pub trait Node {
    fn print(&mut self) -> u32;  // Executes node event, returns ID of next node
    fn connect(&mut self, id: u32); // Sets next node to the given ID
}

struct DNode {
    next_node: u32,
    text: String,
}
impl Node for DNode {
    fn print(&mut self) -> u32 {
        println!("{}", self.text);
        self.next_node
    }
    fn connect(&mut self, id: u32) {
        self.next_node = id;
    }
}

struct QNode {
    next_node: u32,
    text: String,
    options: HashMap<String, u32>,
}
impl Node for QNode {
    fn print(&mut self) -> u32 {
        println!("{}", self.text);
        for (key, _) in &self.options {
            println!("[ {} ]", key);
        }
        let mut user_input = "".to_string();

        // NOTE: Find way to order options? HashMap is unordered data struct

        while !self.options.contains_key(&user_input) {
            io::stdin().read_line(&mut user_input).expect("Failed to read line.");
        }
        let selected: Option<&u32> = self.options.get(&user_input);
        match selected {
            Some(x) => {
                self.next_node = *x;
            }
            None => {
                // Shouldn't occur, print this as a warning instead
                println!("WARNING: Non-existent QNode option selected?");
            }
        }
        self.next_node
    }
    fn connect(&mut self, id: u32) {
        self.next_node = id;
    }
}
impl QNode {
    fn insert_option(&mut self, key: String, value: u32) {
        // NOTE: Can override previously-existing values.
        self.options.insert(key, value);
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

    let mut node_4 = QNode {
        next_node: 0,
        text: String::from("What is your favourite flavour of ice cream?"),
        options: HashMap::new(),
    };
    node_4.insert_option("Vanilla".to_string(), 5);
    node_4.insert_option("Chocolate".to_string(), 6);
    node_4.insert_option("Strawberry".to_string(), 7);
    conversation.insert_node(4, Box::new(node_4));
    conversation.connect_nodes(3, 4);

    let node_5 = DNode {
        next_node: 0,
        text: String::from("Vanilla ice cream? Really? A bit plain, but I suppose it's a valid option."),
    };
    conversation.insert_node(5, Box::new(node_5));

    let node_6 = DNode {
        next_node: 0,
        text: "Chocolate is a very interesting choice!".to_string(),
    };
    conversation.insert_node(6, Box::new(node_6));

    let node_7 = DNode {
        next_node: 0,
        text: "No way! Strawberry is my favourite too!".to_string(),
    };
    conversation.insert_node(7, Box::new(node_7));

    // Print out the dialogue
    conversation.talk()
}
