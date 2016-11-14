extern crate replace_with;

mod unsound {
    use replace_with::replace_with;

    fn sad<T>(_: T) -> T {
        panic!()
    }

    struct MyBox<T> {
        ptr: *mut T,
    }

    impl<T> MyBox<T> {
        fn new(t: T) -> Self {
            MyBox {
                ptr: Box::into_raw(Box::new(t)),
            }
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            unsafe {
                Box::from_raw(self.ptr);
            }
        }
    }

    #[test]
    fn wat() {
        let mut rc = MyBox::new(5);
        replace_with(&mut rc, sad);
    }
}
