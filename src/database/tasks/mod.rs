pub mod read_chunk;
pub mod write_chunk;
pub mod consolidate;
pub mod query;


pub trait Task {
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>);
}