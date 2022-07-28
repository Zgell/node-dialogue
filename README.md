# Node-based Dialogue System

## Introduction
This GitHub repo contains the code for a proof-of-concept dialogue system to be used in game design applications. It uses a node-based system to organize and structure the dialogue. Using nodes allows the coder to seamlessly create non-linear dialogue that can circle back on itself, or behave dynamically (ie. the contents of the dialogue can change based off of user decisions). It's designed to be very easily expandable and handle a wide variety of behaviours so that it can be used in many different settings. It is also designed to make implementing dialogue as simple as possible, by allowing the coder to write the dialogue in a plaintext format outside of the intended application, import it, and have the dialogue system automatically construct a node graph for the conversation from the contents of the file.

## How it Works
Every line of dialogue in a conversation can be modelled with its own individual node. There are a number of different types of nodes that handle certain situations. Each node is connected together in the form of a directed graph. Internally, this is represented just by an unordered map (Rust's `HashMap` struct) that maps an internal "Node ID" to its respective Node object. Each node contains a copy of the ID of the node that it is effectively pointing to. 

All of these nodes are contained within an instance of the `Dialogue` struct, which is a container for the dialogue node tree that includes some additional functionality for constructing the tree. When the `Dialogue` struct's `talk()` method is called, it begins the dialogue and reads through as specified by the node tree. It works by beginning with the node whose ID is 1, executes the node (which means different things based off the type of the node), retrieves what the next node should be from the current node, and repeats the process until the dialogue is finished. The node ID "0" is reserved as a sort of "null node" that is used as the termination condition for the dialogue (that is, when the next node ID is 0, the dialogue will end).

There are several types of dialogue nodes that each perform slightly different functions. The unique functions that each node type performs is the definition of what it means to "execute" a node. Usually, this means reading out the contents of the node to the console (for the sake of this project) and also possibly performing another operation. The primary node is the `DNode` node, shorthand for **Dialogue Node**, and it performs one and only one simple task: print out the contents of the node upon execution. It is used for static dialogue that serves no additional purpose. It has only two fields: a `next_node` field for an integer that represents the ID of the next node in the tree, and a `text` field whose contents will be printed upon execution.

On the other hand we have the `QNode`, or the **Question Node**, which is a specialized node that prints out its contents and then takes in user input from the command line. This node contains a small, finite number of valid answers specified by the coder that is stored inside a `Vec` (dynamic array). The user has the ability to choose from one of these options, and depending on the choice, the question node will point to whichever respective node is connected to the option chosen. This allows for dialogue to change based off of the user's input. Internally, the `QNode` is a bit more complicated than the default `DNode` as it contains a few extra fields: in addition to the usual `next_node` and `text` fields, there are the `options` (a vector for strings) and `options_indices` (unordered map that maps from strings to integers). The `options` field represents an ordered set of options the user can pick from. The `options_indices` hashmap maps all values from the `options` field to the IDs of the nodes they connect to (in an unordered fashion). Upon execution, the contents of `text` will be printed just like with the `DNode`, but then the contents of the `options` field will be printed out one-by-one and the user will be prompted to pick one. Upon selecting a valid answer, the `next_node` field will internally be set to whichever ID value corresponds with the option picked, and that node will be the next one in the tree.

*I also intend to implement an `ENode` struct, which stands for **Event Node**. This node makes a bit more context within a game, since the event node would be used to trigger some kind of in-game event (ex. giving the player an item, triggering a cutscene). The `ENode` doesn't do much on its own in the context of this repo.*

*In the near future, the Dialogue class will also have the ability to import the contents of a dialogue from a plaintext file, parse the file and automatically construct a node tree based off of its contents. Before I can do this however, I must standardize the format of the plaintext file. Some ideas off the top of my head include:*
- *JSON Format*
- *Some custom format where the line number in the plaintext file corresponds to the ID of the node*

## Project Retrospective
This project came to be from a combination of an interest in under-the-hood mechanics in video games and a desire to learn how to use the Rust programming language. I have had this idea for a dialogue system for a while, and wanted to finally code an implementation of it to see how well it works in practice. Rust was an interesting programming language to implement this in, given its relationship with memory management. Fortunately, the planned architecture of the system works quite well with Rust's best and most idiomatic practices, so working with it felt relatively natural. The centralization of the node access is mostly done to comply with Rust's memory-safe design, but I still would have likely followed this architecture for the sake of being able to access any node arbitrarily.

## To-Do
- [x] Get the `Dialogue` struct completed
- [x] Complete the implementation of the default node behaviour (see the `Node` trait)
- [x] Write the `DNode` struct (vanilla, dialogue-only node)
- [x] Write the `QNode` struct ("question node", asks for user input)
- [ ] Write an example of an `ENode`struct ("event node", causes external event to occur)
	- This struct is highly situational and will probably need a custom version for every event case out there, so I will only implement a default/example struct to show how it works.
- [ ] Refactor code to move structs into their own respective files, *and possibly make this program into a Rust package*
- [ ] Come up with a standardized plaintext format that describes all of these node relations
- [ ] Write a plaintext parser that allows you to automatically construct a dialogue node graph from a valid text file.
	- Written examples of dialogue included

