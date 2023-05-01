use nnapi_sys::{
    ANeuralNetworksBurst, ANeuralNetworksBurst_create, ANeuralNetworksBurst_free, ResultCode,
};
use std::{
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
};

use crate::{Compilation, IntoResult};

pub struct Burst {
    inner: NonNull<ANeuralNetworksBurst>,
}

impl Burst {
    pub fn new(compilation: &mut Compilation) -> crate::Result<Burst> {
        let mut burst = null_mut();
        unsafe { ANeuralNetworksBurst_create(&mut **compilation, &mut burst) }.into_result()?;

        Ok(Burst {
            inner: NonNull::new(burst).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?,
        })
    }
}

impl Deref for Burst {
    type Target = ANeuralNetworksBurst;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

impl DerefMut for Burst {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut() }
    }
}

impl Drop for Burst {
    fn drop(&mut self) {
        unsafe {
            ANeuralNetworksBurst_free(self.inner.as_mut());
        }
    }
}
