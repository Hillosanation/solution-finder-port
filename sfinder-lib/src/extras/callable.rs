// This is to transfer the Callable interface
// TODO(#13): deprecate this trait? This seems to be a workaraound in Java for lazy evaluation
pub trait Callable<T> {
    fn call(&self) -> T;
}
