use odin_kernel::KernelError;

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}
