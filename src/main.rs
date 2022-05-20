enum Tree {
    Empty,
    Node { l : Box<Tree>, r : Box<Tree> },
}

struct Set {
    len : u64,
    rep : Box<Tree>
}

fn new() -> Set {
    Set {
        len : 0,
        rep : Box::new(Tree::Empty),
    }
}

fn main() {
    let mut _c = new();
}
 