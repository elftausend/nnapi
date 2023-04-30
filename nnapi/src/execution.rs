use std::{ptr::{null_mut, NonNull}, ops::{Deref, DerefMut}, mem::size_of};

use nnapi_sys::{ANeuralNetworksExecution, ANeuralNetworksExecution_create, ResultCode, ANeuralNetworksExecution_free, ANeuralNetworksExecution_setInput, ANeuralNetworksExecution_setOutput};

use crate::{Compilation, IntoResult};

pub struct Execution {
    inner: NonNull<ANeuralNetworksExecution>,
}

impl Execution {
    pub fn new(compilation: &mut Compilation) -> crate::Result<Self> {
        let mut execution = null_mut();

        unsafe { ANeuralNetworksExecution_create(&mut **compilation, &mut execution) }
            .into_result()?;

        Ok(Execution {
            inner: NonNull::new(execution).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?,
        })
    }

    #[inline]
    pub fn set_input<T>(&mut self, input_list_idx: i32, input: &[T]) -> crate::Result<()> {
        unsafe { ANeuralNetworksExecution_setInput(&mut **self, input_list_idx, null_mut(), input.as_ptr().cast(), input.len() * size_of::<T>()) }
            .into_result()
    }

    #[inline]
    pub fn set_output<T>(&mut self, output_list_idx: i32, output: &mut [T]) -> crate::Result<()> {
        unsafe { ANeuralNetworksExecution_setOutput(&mut **self, output_list_idx, null_mut(), output.as_mut_ptr().cast(), output.len() * size_of::<T>()) }
            .into_result()
    }
}

impl Deref for Execution {
    type Target = ANeuralNetworksExecution;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {self.inner.as_ref()}
    }
}

impl DerefMut for Execution {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {self.inner.as_mut()}
    }
}

impl Drop for Execution {
    fn drop(&mut self) {
        unsafe {
            ANeuralNetworksExecution_free(self.inner.as_mut());
        }
    }
}
