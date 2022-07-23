use std::collections::HashMap;

struct Node {
    text: String,  // Contents of dialogue for this one node
    next_node: u32,  // ID of next node to jump to
}

impl Node {
    fn print(&self) {
        println!("{}", self.text);
    }
    fn connect(&mut self, id: u32) {
        self.next_node = id;
    }
    fn get_next_node(&self) -> u32{
        self.next_node
    }
}

struct Dialogue {
    filepath: String,  // File that dialogue script is loaded from
    nodes: HashMap<u32, Node>,
}

impl Dialogue {
    fn print(&self) {
        /*
        Reads the conversation as intended.
        */
        // First, fetch node of ID 1 and read it
        let mut next_node: u32 = 1;
        while next_node != 0 {
            let current_node = &self.nodes.get(&next_node);
            match current_node {
                Some(node) => {
                    node.print();
                    next_node = node.get_next_node();
                },
                None => {
                    // End the conversation
                    next_node = 0;
                }
            }
        }
    }
    fn print_nodes(&self) {
        /* 
        Prints out the dialogue nodes in an ARBITRARY order.
        Used strictly for debugging purposes. 
        */
        for (_, value) in &self.nodes {
            value.print();
        }
    }
    fn insert_node(&mut self, key: u32, value: Node) {
        /*
        Inserts a node into the structure.
        Equivalent to "Dialogue.nodes.insert(key, value)".
        */
        self.nodes.insert(key, value);
    }
    fn connect_nodes(&mut self, from_id: u32, to_id: u32) {
        /*
        Attaches the output of node FROM_ID to the input of node TO_ID.
        Ie. In Conversation: Node(FROM_ID) --> Node(TO_ID)
        */
        let node_wrapped = self.nodes.get_mut(&from_id);
        match node_wrapped {
            Some(node) => {
                node.connect(to_id);
            },
            None => {},
        }
        // let node = self.nodes.get(&from_id);
        // if node.is_some() {
        //     node.unwrap().connect(&to_id);
        // }
    }
}

// enum YesNo {
//     Yes,
//     No,
// }



fn main() {
    // Node Test
    // let myvec = vec![1, 2, 5];
    // println!("{}, {}, {}", myvec[0], myvec[1], myvec[2]);
    // println!("{:?}", inventory);
    let mut convo = Dialogue {
        filepath: String::from("C:/FAKEPATH"),
        nodes: HashMap::new(),
    };

    // Manually create a conversation for testing purposes
    let node_1 = Node {
        text: String::from("Hello, and welcome to the test dialogue!"),
        next_node: 0,
    };
    // convo.nodes.insert(1, node_1);
    convo.insert_node(1, node_1);
    let node_2 = Node {
        text: String::from("This is an example used to demonstrate how you can use this dialogue system."),
        next_node: 0,
    };
    // convo.nodes.insert(2, node_2);
    convo.insert_node(2, node_2);
    convo.connect_nodes(1, 2);
    let node_3 = Node {
        text: String::from("You can also answer questions and prompts too!"),
        next_node: 0,
    };
    // convo.nodes.insert(3, node_3);
    convo.insert_node(3, node_3);
    convo.connect_nodes(2, 3);

    // Print dialogue
    println!("[ CONVERSATION ]");
    convo.print();
}
