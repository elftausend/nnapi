use std::{ptr::NonNull, ops::{Deref, DerefMut}};

use nnapi_sys::{ANeuralNetworksEvent, ANeuralNetworksEvent_wait};

use crate::IntoResult;

pub struct Event {
    pub(crate) inner: NonNull<ANeuralNetworksEvent>,
}

impl Event {
    pub fn wait(&mut self) -> crate::Result<()> {
        unsafe { ANeuralNetworksEvent_wait(&mut **self) }
            .into_result()
    }
}

impl Deref for Event {
    type Target = ANeuralNetworksEvent;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

impl DerefMut for Event {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut() }
    }
}
