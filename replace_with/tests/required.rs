extern crate replace_with;

pub use replace_with::replace_with;

// This macro is an assertion with nicely formatted failure output
macro_rules! assert_expected_eq_actual {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(*a == *b,
                "\nExpected `{:?}` is not equal to Actual `{:?}`\nAssertion: `assert_expected_eq_actual!({}, {})`",
                *a,
                *b,
                stringify!($a),
                stringify!($b));
    })
}

mod required {

    use super::replace_with;

    use std::mem;

    #[test]
    fn simple_type() {
        let mut x = Box::new(6);
        assert_expected_eq_actual!(*x, 6);
        replace_with(&mut x, |mut b: Box<i32>| {
            *b += 1;
            b
        });
        assert_expected_eq_actual!(*x, 7);
        mem::drop(x);   // Takes ownership of x.
    }

    #[derive(Debug,PartialEq)]
    enum Tree {
        Leaf(i32),
        Fork(Box<Tree>, Box<Tree>),
    }

    #[test]
    fn complex_type() {
        let mut x = Tree::Leaf(0);
        assert_expected_eq_actual!(x, Tree::Leaf(0));
        replace_with(&mut x, |t: Tree| {
            Tree::Fork(Box::new(t), Box::new(Tree::Leaf(1)))
        });
        let expected = Tree::Fork(Box::new(Tree::Leaf(0)), Box::new(Tree::Leaf(1)));
        assert_expected_eq_actual!(x, expected);
        mem::drop(x);   // Takes ownership of x.
    }

    #[test]
    #[ignore]
    fn make_error() {
        // Make a thread that shares some mutable data that has a Drop that
        // frees memory. Then use replace_with with a panicking function. The
        // panic will stop the replace_with halfway through and cause the value
        // to be left in an unstable state. Then when we try to destruct the
        // original vector, we get a segmentation fault.

        use std::thread;
        use std::sync::Mutex;
        use std::sync::Arc;
        let data = Arc::new(Mutex::new(vec![1]));

        let _ = thread::spawn(move || {
            use std::ops::DerefMut;
            let mut val = data.lock().unwrap();
            let mut dataref = val.deref_mut();;
            replace_with(dataref, |_| panic!());
        }).join();
    }

    #[test]
    #[ignore]
    fn make_simple_error() {
        // I made this one after looking at the unsound branch
        let mut x = Box::new(5);
        replace_with(&mut x, |_| panic!());
    }
}

