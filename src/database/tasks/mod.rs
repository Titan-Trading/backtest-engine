pub mod read_chunk;
pub mod write_chunk;
pub mod consolidate;
pub mod perform_query;


pub trait Task {
    // execute the task and setup optional on exit callback
    fn execute(&mut self, on_exit: Option<Box<dyn FnOnce(bool) + Send + 'static>>);

    // end the task
    fn close(&mut self);
}