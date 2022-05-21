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
        // keep track of current pointer
        let mut curr = &mut self.rep;
        for i in (0..8).rev() {
            // extract i'th bit
            let indexer = 1 << i;
            let dir = if (item & indexer) == 0 { 0 } else { 1 };

            // create new path if we hit a dead-end
            if let &mut Tree::Empty(_) = curr {
                *curr = Tree::Node(Box::new(Tree::Empty(false)), Box::new(Tree::Empty(false)));
            }

            // have pointer follow path
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

    fn delete(&mut self, item: u8) {
        // keep track of current pointer
        let mut curr = &mut self.rep;
        for i in (0..8).rev() {
            // extract i'th bit
            let indexer = 1 << i;
            let dir = if (item & indexer) == 0 { 0 } else { 1 };

            // do nothing if we hit a dead-end
            if let &mut Tree::Empty(_) = curr {
                return;
            }

            // have pointer follow path
            if let &mut Tree::Node(ref mut l, ref mut r) = curr {
                curr = if dir == 0 { l } else { r };
            }
        }

        // check if current element is in tree
        if let &mut Tree::Empty(true) = curr {
            *curr = Tree::Empty(false);
            self.len -= 1;
        }
    }

    fn exists(&self, item: u8) -> bool {
        // keep track of current pointer
        let mut curr = &self.rep;
        for i in (0..8).rev() {
            // extract i'th bit
            let indexer = 1 << i;
            let dir = if (item & indexer) == 0 { 0 } else { 1 };

            // do nothing if we hit a dead-end
            if let &Tree::Empty(_) = curr {
                return false;
            }

            // have pointer follow path
            if let &Tree::Node(ref l, ref r) = curr {
                curr = if dir == 0 { l } else { r };
            }
        }

        // check if current element is in tree
        if let &Tree::Empty(true) = curr {
            return true;
        }
        return false;
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

    println!("--------");
    c.delete(0);
    c.print();

    println!("--------");
    c.add(8);
    c.delete(32);
    c.print();

    println!("--------");
    println!("32 exists in set?: {}", c.exists(32));
    println!("8 exists in set?: {}", c.exists(8));
    println!("0 exists in set?: {}", c.exists(0));
    println!("7 exists in set?: {}", c.exists(7));
}
