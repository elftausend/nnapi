use nnapi_sys::{ANeuralNetworksOperandType, OperandCode};

#[derive(Debug, Clone, Copy)]
pub struct Operand {
    pub inner: ANeuralNetworksOperandType,
}

impl Operand {
    pub fn tensor(dtype: OperandCode, dimensions: &[u32], scale: f32, zero_point: i32) -> Self {
        Operand {
            inner: ANeuralNetworksOperandType {
                type_: dtype as i32,
                dimensionCount: dimensions.len() as u32,
                dimensions: dimensions.as_ptr(),
                scale,
                zeroPoint: zero_point,
            },
        }
    }

    #[inline]
    pub fn activation() -> Self {
        Operand {
            inner: ANeuralNetworksOperandType {
                type_: OperandCode::ANEURALNETWORKS_INT32 as i32,
                dimensionCount: 0,
                dimensions: std::ptr::null_mut(),
                scale: 0.,
                zeroPoint: 0,
            },
        }
    }
}
