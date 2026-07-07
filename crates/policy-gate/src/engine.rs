use odin_kernel::KernelError;

pub struct PolicyGate;

impl PolicyGate {
    pub fn new() -> Self {
        Self
    }

    pub fn enforce(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for PolicyGate {
    fn default() -> Self {
        Self::new()
    }
}
