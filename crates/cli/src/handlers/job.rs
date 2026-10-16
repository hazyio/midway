use std::thread::{self, JoinHandle};
pub struct Job {
    pending: Vec<JoinHandle<()>>,
}
impl Job {
    pub fn create() -> Self {
        Job {
            pending: Vec::new(),
        }
    }
    pub fn new_job<F>(&mut self,  f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pending.push(thread::spawn(move || {
            f();
        }));
    }
    pub fn wait_all(&mut self) {
        while let Some(handle) = self.pending.pop() {
            let _ = handle.join();
        }
    }
}
