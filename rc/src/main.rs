extern crate my_rc;
use my_rc::MyRc;
use std::cell::Cell;
use std::rc::Rc;

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

struct D {
        d_count: Rc<Cell<i32>>,
    }

    impl Drop for D {
        fn drop(&mut self) {
            self.d_count.set(self.d_count.get() + 1);
        }
    }

fn main() {
        let keep = {
            let rc = MyRc::new(5);
                {
                    let rc2 = rc.clone();
                }
            rc.consume()
        };
        println!("{}", keep.is_ok());
}