use std::{
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
};

use nnapi_sys::{
    ANeuralNetworksCompilation, ANeuralNetworksCompilation_create, ANeuralNetworksCompilation_free,
    ResultCode, ANeuralNetworksCompilation_finish,
};

use crate::{IntoResult, Model, Execution};

pub struct Compilation {
    inner: NonNull<ANeuralNetworksCompilation>,
}

impl Compilation {
    pub fn new(model: &mut Model) -> crate::Result<Self> {
        let mut compilation = null_mut();
        unsafe { ANeuralNetworksCompilation_create(&mut **model, &mut compilation) }
            .into_result()?;

        Ok(Compilation {
            inner: NonNull::new(compilation).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?,
        })
    }

    #[inline]
    pub fn finish(&mut self) -> crate::Result<()> {
        unsafe { ANeuralNetworksCompilation_finish(&mut **self) }.into_result()
    }

    #[inline]
    pub fn create_execution(&mut self) -> crate::Result<Execution> {
        crate::Execution::new(self)
    }
}

impl Deref for Compilation {
    type Target = ANeuralNetworksCompilation;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

impl DerefMut for Compilation {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut() }
    }
}

impl Drop for Compilation {
    fn drop(&mut self) {
        unsafe {
            ANeuralNetworksCompilation_free(self.inner.as_mut());
        }
    }
}
