use crate::vm::runners::cairo_runner::CairoRunner;
use crate::vm::vm_core::VirtualMachine;

pub trait RunnerPreStepHook {
    fn execute(&self, vm: &VirtualMachine);
}

impl CairoRunner {
    pub fn set_pre_step_hook(&mut self, pre_step_hook: impl RunnerPreStepHook + 'static) {
        self.pre_step_hook = Some(Box::new(pre_step_hook));
    }

    pub fn execute_pre_step_hook(&self) {
        if let Some(hook) = &self.pre_step_hook {
            hook.execute(&self.vm);
        }
    }
}
