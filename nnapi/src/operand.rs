use nnapi_sys::{ANeuralNetworksOperandType, OperandCode};

#[derive(Debug, Clone)]
pub struct Operand {
    pub inner: ANeuralNetworksOperandType,
    pub len: usize,
    pub dimensions: Vec<u32>
}

impl Operand {
    pub fn tensor(dtype: OperandCode, dimensions: Vec<u32>, scale: f32, zero_point: i32) -> Self {
        Operand {
            inner: ANeuralNetworksOperandType {
                type_: dtype as i32,
                dimensionCount: dimensions.len() as u32,
                dimensions: dimensions.as_ptr(),
                scale,
                zeroPoint: zero_point,
            },
            len: dimensions.iter().product::<u32>() as usize,
            dimensions
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
            len: 0,
            dimensions: vec![],
        }
    }
}
