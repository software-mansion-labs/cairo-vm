use crate::vm::errors::vm_errors::VirtualMachineError;
use crate::vm::runners::cairo_runner::CairoRunner;
use crate::vm::vm_core::VirtualMachine;

pub trait RunnerPreStepHook {
    fn execute(&mut self, vm: &VirtualMachine) -> Result<(), VirtualMachineError>;
}

impl CairoRunner {
    pub fn set_pre_step_hook(&mut self, pre_step_hook: impl RunnerPreStepHook + 'static) {
        self.pre_step_hook = Some(Box::new(pre_step_hook));
    }

    pub fn execute_pre_step_hook(&mut self) -> Result<(), VirtualMachineError> {
        if let Some(hook) = &mut self.pre_step_hook {
            hook.execute(&self.vm)?;
        }

        Ok(())
    }
}
