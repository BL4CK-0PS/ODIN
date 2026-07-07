use odin_kernel::KernelError;

pub struct MemoryEngine;

impl MemoryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn store(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for MemoryEngine {
    fn default() -> Self {
        Self::new()
    }
}
