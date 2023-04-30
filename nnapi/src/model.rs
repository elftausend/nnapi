use std::{ptr::{null_mut, NonNull}, ops::{Deref, DerefMut}};

use nnapi_sys::{
    ANeuralNetworksModel, ANeuralNetworksModel_create, ANeuralNetworksModel_free, ResultCode, ANeuralNetworksModel_addOperand, ANeuralNetworksModel_setOperandValue, ANEURALNETWORKS_FUSED_NONE,
};

use crate::{
    error::{IntoResult, NnapiResult},
    Operand,
};

pub struct Model {
    pub model: NonNull<ANeuralNetworksModel>,
}

impl Model {
    pub fn new() -> NnapiResult<Self> {
        let mut model: *mut ANeuralNetworksModel = null_mut();

        unsafe { ANeuralNetworksModel_create(&mut model) }.into_result()?;

        let model = NonNull::new(model).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?;

        Ok(Self { model })
    }

    #[inline]
    pub fn add_operand(&mut self, operand: Operand) -> NnapiResult<()> {
        unsafe { ANeuralNetworksModel_addOperand(self.model.as_mut(), &operand.inner) }.into_result()
    }

    #[inline]
    pub fn set_activation_operand_value(&mut self, activation_idx: i32) -> NnapiResult<()> {
        let none_value = ANEURALNETWORKS_FUSED_NONE;
        unsafe {
            ANeuralNetworksModel_setOperandValue(
                &mut **self,
                activation_idx,
                &none_value as *const _ as *const _,
                4,
            )
        }.into_result()
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
