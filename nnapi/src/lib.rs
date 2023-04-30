mod compilation;
mod error;
mod execution;
mod model;
mod operand;
mod event;

pub use compilation::*;
pub use error::*;
pub use execution::*;
pub use model::*;
pub use operand::*;
pub use event::*;

pub trait AsOperandCode {
    const OPERAND_CODE: nnapi_sys::OperandCode;
}

impl AsOperandCode for f32 {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_FLOAT32;
}

impl AsOperandCode for i32 {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_INT32;
}

impl AsOperandCode for u32 {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_UINT32;
}

impl AsOperandCode for bool {
    const OPERAND_CODE: nnapi_sys::OperandCode = nnapi_sys::OperandCode::ANEURALNETWORKS_TENSOR_BOOL8;
}
