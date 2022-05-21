enum Tree {
    Empty(bool),
    Node(Box<Tree>, Box<Tree>),
}

struct Set {
    len: u64,
    rep: Tree,
}

impl Tree {
    fn print(&self, prefix: &str) {
        if let &Tree::Empty(true) = self {
            println!("{}", prefix);
            return;
        }
        if let &Tree::Node(ref l, ref r) = self {
            let l_str = &(String::from(prefix) + "0");
            let r_str = &(String::from(prefix) + "1");
            l.print(l_str);
            r.print(r_str);
        }
    }
}

impl Set {
    fn new() -> Set {
        Set {
            len: 0,
            rep: Tree::Empty(false),
        }
    }

    fn print(&self) {
        println!("Len: {}", self.len);
        self.rep.print("");
    }

    fn add(&mut self, item: u8) {
        let mut curr = &mut self.rep;

        for i in (0..8).rev() {
            let indexer = 1 << i;
            let dir = if (item & indexer) == 0 { 0 } else { 1 };

            if let &mut Tree::Empty(_) = curr {
                *curr = Tree::Node(Box::new(Tree::Empty(false)), Box::new(Tree::Empty(false)));
            }

            if let &mut Tree::Node(ref mut l, ref mut r) = curr {
                curr = if dir == 0 { l } else { r };
            }
        }

        // check if current element is not already in tree
        if let &mut Tree::Empty(false) = curr {
            self.len += 1;
            *curr = Tree::Empty(true);
        }
    }
}

fn main() {
    let mut c = Set::new();
    c.add(5);
    c.print();

    println!("--------");
    c.add(6);
    c.print();

    println!("--------");
    c.add(7);
    c.add(0);
    c.add(19);
    c.add(32);
    c.add(0);
    c.print();
}
