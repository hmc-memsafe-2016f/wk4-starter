extern crate my_rc;

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

#[allow(unused_variables)]
mod required {
    use my_rc::MyRc;
    use std::cell::Cell;
    use std::rc::Rc;
    use my_rc::MyRefCounter;

    #[test]
    fn new_exists() {
        let x = MyRc::new(5);
    }

    #[test]
    fn consume_ok() {
        let x = MyRc::new(5);
        let b = x.consume();
        assert!(b.is_ok())
    }

    #[test]
    fn clone_exist() {
        let c = MyRc::new(5);
        let cc = c.clone();
    }

    #[test]
    fn consume_err() {
        let x = MyRc::new(5);
        let y = x.clone();
        let b = x.consume();
        assert!(b.is_err());
    }

    #[test]
    fn deref() {
        let x = MyRc::new(5);
        let y = x.clone();
        assert_expected_eq_actual!(5, *x);
        assert_expected_eq_actual!(5, *y);
    }

    #[test]
    fn deref_identity() {
        let x = MyRc::new(Cell::new(5));
        let y = x.clone();
        y.set(6);
        assert_expected_eq_actual!(6, x.get());
    }


    struct D {
        d_count: Rc<Cell<i32>>,
    }

    impl Drop for D {
        fn drop(&mut self) {
            self.d_count.set(self.d_count.get() + 1);
        }
    }

    #[test]
    fn destrutors_run() {
        let d_count = Rc::new(Cell::new(0));
        {
            let rc = MyRc::new(D { d_count: d_count.clone() });
            {
                let rc2 = rc.clone();
                assert_expected_eq_actual!(0, d_count.get());
            }
            assert_expected_eq_actual!(0, d_count.get());
        }
        assert_expected_eq_actual!(1, d_count.get());
    }

    #[test]
    fn destrutors_and_consume() {
        let d_count = Rc::new(Cell::new(0));
        let d = {
            let rc = MyRc::new(D { d_count: d_count.clone() });
            assert_expected_eq_actual!(0, d_count.get());
            let x = rc.consume();
            x
        };
        assert_expected_eq_actual!(0, d_count.get());
    }
}
