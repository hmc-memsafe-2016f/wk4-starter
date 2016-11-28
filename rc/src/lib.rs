mod my_rc;

pub use my_rc::MyRc;

pub trait MyRefCounter<T> {
    fn new(t: T) -> MyRc<T>;
    fn consume(self) -> Result<T,MyRc<T>>;
}
