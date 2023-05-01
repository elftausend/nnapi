//! # Safe and abstracted Rust bindings for Android Neural Networks API

mod burst;
mod compilation;
mod error;
mod event;
mod execution;
mod model;
mod operand;

pub use burst::*;
pub use compilation::*;
pub use error::*;
pub use event::*;
pub use execution::*;
pub use model::*;
pub use operand::*;
pub mod nnapi_sys {
    pub use ::nnapi_sys::*;
}

pub trait AsOperandCode {
    const OPERAND_CODE: nnapi_sys::OperandCode;
}

/// not useable for tensors!
impl AsOperandCode for f32 {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_TENSOR_FLOAT32;
}

impl AsOperandCode for i32 {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_TENSOR_INT32;
}

impl AsOperandCode for u32 {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_UINT32;
}

impl AsOperandCode for bool {
    const OPERAND_CODE: nnapi_sys::OperandCode =
        nnapi_sys::OperandCode::ANEURALNETWORKS_TENSOR_BOOL8;
}
