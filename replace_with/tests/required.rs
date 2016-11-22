extern crate replace_with;

pub use replace_with::replace_with;

mod required {

    use super::replace_with;

    use std::mem;

    #[test]
    fn simple_type() {
        let mut x = Box::new(6);
        replace_with(&mut x, |mut b: Box<i32>| {
            *b += 1;
            b
        });
        mem::drop(x);   // Takes ownership of x.
    }

    enum Tree {
        Leaf(i32),
        Fork(Box<Tree>, Box<Tree>),
    }

    #[test]
    fn complex_type() {
        let mut x = Tree::Leaf(0);
        replace_with(&mut x, |t: Tree| {
            Tree::Fork(Box::new(t), Box::new(Tree::Leaf(1)))
        });
        mem::drop(x);   // Takes ownership of x.
    }

    #[test]
    #[ignore]
    fn safe_memory_error() {
        use std::rc::Rc;
        let data = vec![1];
        let mut val = Rc::new(data);
        replace_with(&mut val, |_| panic!());
    }
}
