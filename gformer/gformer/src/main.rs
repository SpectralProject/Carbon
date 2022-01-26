// Bluprints: gameforger version of UE4 blueprints
struct Bluprint {
    root: Node,
    size: u32
}

// Node based system
struct Node {
    children: Vec<Node>,
    parents: Vec<Node>
}

fn main() {
    println!("Hello, world!");
}
