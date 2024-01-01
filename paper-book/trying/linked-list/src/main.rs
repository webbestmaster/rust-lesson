// use std::cell::RefCell;

pub mod node;

fn main() {
    println!("Linked list:");

    let mut node = node::Node {
        name: "Earl".to_string(),
        next: None,
    };

    node.next = Some(Box::new(node::Node {
        name: "Doraty".to_string(),
        next: None,
    }));

    println!("node {:?}", node.next);

    match node.next {
        Some(ref node) => println!("{:?}", node),
        None => println!("none"),
    }

    let last_node = node.get_first();

    println!("first node {:?}", last_node);

    match node.next {
        Some(ref mut node) => {
            node.set_next(Some(Box::new(node::Node {
                name: "Trinity".to_string(),
                next: None,
            })));
        }
        None => println!("none"),
    }

    println!("node list {:?}", node);
}
