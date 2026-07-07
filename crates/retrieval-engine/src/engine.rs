use odin_kernel::KernelError;

pub struct RetrievalEngine;

impl RetrievalEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn query(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for RetrievalEngine {
    fn default() -> Self {
        Self::new()
    }
}
