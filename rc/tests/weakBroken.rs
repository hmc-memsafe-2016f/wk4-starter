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
    use my_rc::MyBrokenRc;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::mem;

    #[test]
    fn new_exists() {
        let x = MyBrokenRc::new(5);
    }

    #[test]
    fn downgrade_exists() {
        let x = MyBrokenRc::new(5);
        {
            let y = x.downgrade();
            let z = x.downgrade();
            let xx = z.upgrade().expect("Should be valid");
            assert_expected_eq_actual!(5, *xx);
        }
        assert_expected_eq_actual!(5, *x);
    }

    #[test]
    fn downgrade_fails() {
        let x = MyBrokenRc::new(5);
        let y = x.downgrade();
        mem::drop(x);
        assert!(y.upgrade().is_none());
    }

    #[test]
    fn consume_ok() {
        let x = MyBrokenRc::new(5);
        let b = x.consume();
        assert!(b.is_ok())
    }

    #[test]
    fn clone_exist() {
        let c = MyBrokenRc::new(5);
        let cc = c.clone();
    }

    #[test]
    fn consume_err() {
        let x = MyBrokenRc::new(5);
        let y = x.clone();
        let b = x.consume();
        assert!(b.is_err());
    }

    #[test]
    fn deref() {
        let x = MyBrokenRc::new(5);
        let y = x.clone();
        assert_expected_eq_actual!(5, *x);
        assert_expected_eq_actual!(5, *y);
    }

    #[test]
    fn deref_identity() {
        let x = MyBrokenRc::new(Cell::new(5));
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
            let rc = MyBrokenRc::new(D { d_count: d_count.clone() });
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
            let rc = MyBrokenRc::new(D { d_count: d_count.clone() });
            assert_expected_eq_actual!(0, d_count.get());
            rc.consume()
        };
        assert_expected_eq_actual!(0, d_count.get());
    }

    #[test]
    fn destrutors_orders() {
        let x = MyBrokenRc::new(5);
        let xx = x.clone();
        let y = x.downgrade();
        let yy = xx.downgrade();
        let yyy = xx.downgrade();
        mem::drop(y);
        mem::drop(yy.upgrade());
        mem::drop(x);
        mem::drop(xx);
        mem::drop(yy);
        mem::drop(yyy);
    }
}
