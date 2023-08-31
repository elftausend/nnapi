use std::{
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
};

use nnapi_sys::{
    ANeuralNetworksModel, ANeuralNetworksModel_addOperand, ANeuralNetworksModel_addOperation,
    ANeuralNetworksModel_create, ANeuralNetworksModel_finish, ANeuralNetworksModel_free,
    ANeuralNetworksModel_identifyInputsAndOutputs, ANeuralNetworksModel_setOperandValue,
    OperationCode, ResultCode, ANEURALNETWORKS_FUSED_NONE,
};

use crate::{
    error::{IntoResult, Result},
    Compilation, Operand,
};

pub struct Model {
    pub model: NonNull<ANeuralNetworksModel>,
}

impl Model {
    pub fn new() -> Result<Self> {
        let mut model: *mut ANeuralNetworksModel = null_mut();

        unsafe { ANeuralNetworksModel_create(&mut model) }.into_result()?;

        let model = NonNull::new(model).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?;

        Ok(Self { model })
    }

    pub fn from_operands(operands: impl IntoIterator<Item = Operand>) -> Result<Self> {
        let mut model = Model::new()?;
        for operand in operands.into_iter() {
            model.add_operand(&operand)?;
        }

        Ok(model)
    }

    #[inline]
    pub fn add_operand(&mut self, operand: &Operand) -> Result<()> {
        unsafe { ANeuralNetworksModel_addOperand(self.model.as_mut(), &operand.inner) }
            .into_result()
    }

    // TODO:: add set operand value for buffers

    #[inline]
    pub fn set_activation_operand_value(&mut self, activation_idx: i32) -> Result<()> {
        let none_value = ANEURALNETWORKS_FUSED_NONE;
        unsafe {
            ANeuralNetworksModel_setOperandValue(
                &mut **self,
                activation_idx,
                &none_value as *const _ as *const _,
                4,
            )
        }
        .into_result()
    }

    #[inline]
    pub fn add_operation(
        &mut self,
        operation: OperationCode,
        inputs: &[u32],
        outputs: &[u32],
    ) -> Result<()> {
        unsafe {
            ANeuralNetworksModel_addOperation(
                &mut **self,
                operation as i32,
                inputs.len().try_into().unwrap(),
                inputs.as_ptr(),
                outputs.len().try_into().unwrap(),
                outputs.as_ptr(),
            )
        }
        .into_result()
    }

    pub fn identify_inputs_and_outputs(&mut self, inputs: &[u32], outputs: &[u32]) -> Result<()> {
        unsafe {
            ANeuralNetworksModel_identifyInputsAndOutputs(
                &mut **self,
                inputs.len().try_into().unwrap(),
                inputs.as_ptr(),
                outputs.len().try_into().unwrap(),
                outputs.as_ptr(),
            )
        }
        .into_result()
    }

    #[inline]
    pub fn finish(&mut self) -> Result<()> {
        unsafe { ANeuralNetworksModel_finish(&mut **self) }.into_result()
    }

    #[inline]
    pub fn compile(&mut self) -> Result<Compilation> {
        Compilation::new(self)
    }
}

impl Deref for Model {
    type Target = ANeuralNetworksModel;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.model.as_ref() }
    }
}

impl DerefMut for Model {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.model.as_mut() }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { ANeuralNetworksModel_free(self.model.as_mut()) };
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::Result, Model};

    #[test]
    fn test_compile() -> Result<()> {
        let mut model = Model::new()?;

        Ok(())
    }
}
