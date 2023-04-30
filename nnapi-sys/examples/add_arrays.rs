use std::{ffi::c_void, ptr::null_mut};

use nnapi_sys::{
    ANeuralNetworksCompilation, ANeuralNetworksCompilation_create,
    ANeuralNetworksCompilation_finish, ANeuralNetworksCompilation_free, ANeuralNetworksEvent,
    ANeuralNetworksEvent_free, ANeuralNetworksEvent_wait, ANeuralNetworksExecution,
    ANeuralNetworksExecution_create, ANeuralNetworksExecution_free,
    ANeuralNetworksExecution_setInput, ANeuralNetworksExecution_setOutput,
    ANeuralNetworksExecution_startCompute, ANeuralNetworksModel, ANeuralNetworksModel_addOperand,
    ANeuralNetworksModel_addOperation, ANeuralNetworksModel_create, ANeuralNetworksModel_finish,
    ANeuralNetworksModel_free, ANeuralNetworksModel_identifyInputsAndOutputs,
    ANeuralNetworksModel_setOperandValue, ANeuralNetworksOperandType, OperandCode,
    ANEURALNETWORKS_ADD, ANEURALNETWORKS_FUSED_NONE,
};

fn main() {
    let tensor9x_type = ANeuralNetworksOperandType {
        type_: OperandCode::ANEURALNETWORKS_TENSOR_FLOAT32 as i32,
        dimensionCount: 1,
        dimensions: [9].as_ptr(),
        scale: 0.,
        zeroPoint: 0,
    };

    let activation_type = ANeuralNetworksOperandType {
        type_: OperandCode::ANEURALNETWORKS_INT32 as i32,
        dimensionCount: 0,
        dimensions: null_mut(),
        scale: 0.,
        zeroPoint: 0,
    };

    let mut model: *mut ANeuralNetworksModel = null_mut();

    if unsafe { ANeuralNetworksModel_create(&mut model) } != 0 {
        panic!("Failed to create model");
    }

    if unsafe { ANeuralNetworksModel_addOperand(model, &tensor9x_type) } != 0 {
        panic!("Failed to add operand 0");
    }
    unsafe { ANeuralNetworksModel_addOperand(model, &tensor9x_type) };
    unsafe { ANeuralNetworksModel_addOperand(model, &activation_type) };
    unsafe { ANeuralNetworksModel_addOperand(model, &tensor9x_type) };

    let lhs_idx = 0;
    let rhs_idx = 1;
    let activation_idx = 2;
    let out_idx = 3;

    let outputs = [out_idx];

    let none_value = ANEURALNETWORKS_FUSED_NONE;

    // set the activation operand to a none value (fixes "Graph contains at least one cycle or one never-written operand")
    if 0 != unsafe {
        ANeuralNetworksModel_setOperandValue(
            model,
            activation_idx as i32,
            &none_value as *const _ as *const _,
            4,
        )
    } {
        panic!("Failed to set activation operand value!");
    }

    if 0 != unsafe {
        // For whatever reason an activation is needed? -> [.., activation_idx]
        let inputs = [lhs_idx, rhs_idx, activation_idx];
        ANeuralNetworksModel_addOperation(
            model,
            ANEURALNETWORKS_ADD,
            3, // inputs.len().try_into().unwrap(),
            inputs.as_ptr(),
            1, // outputs.len().try_into().unwrap(),
            outputs.as_ptr(),
        )
    } {
        panic!("Failed to add add operation!");
    }
    unsafe {
        let inputs = [lhs_idx, rhs_idx];
        ANeuralNetworksModel_identifyInputsAndOutputs(
            model,
            inputs.len().try_into().unwrap(),
            inputs.as_ptr(),
            outputs.len().try_into().unwrap(),
            outputs.as_ptr(),
        )
    };

    unsafe { ANeuralNetworksModel_finish(model) };

    let mut compilation: *mut ANeuralNetworksCompilation = null_mut();
    unsafe { ANeuralNetworksCompilation_create(model, &mut compilation) };

    unsafe { ANeuralNetworksCompilation_finish(compilation) };

    let mut run1: *mut ANeuralNetworksExecution = null_mut();
    unsafe { ANeuralNetworksExecution_create(compilation, &mut run1) };

    let lhs = [1f32, 2., 3., 4., 5., 6., 7., 8., 9.];
    let rhs = [1f32, 2., 3., 4., 5., 6., 7., 8., 9.];

    unsafe {
        ANeuralNetworksExecution_setInput(
            run1,
            0,
            null_mut(),
            lhs.as_ptr() as *const c_void,
            (lhs.len() * std::mem::size_of::<f32>()).try_into().unwrap(),
        )
    };
    unsafe {
        ANeuralNetworksExecution_setInput(
            run1,
            1,
            null_mut(),
            rhs.as_ptr() as *const c_void,
            (rhs.len() * std::mem::size_of::<f32>()).try_into().unwrap(),
        )
    };

    let mut out = [0f32; 9];

    unsafe {
        ANeuralNetworksExecution_setOutput(
            run1,
            0,
            null_mut(),
            out.as_mut_ptr() as *mut c_void,
            (out.len() * std::mem::size_of::<f32>()).try_into().unwrap(),
        )
    };

    let mut run1_end: *mut ANeuralNetworksEvent = null_mut();
    unsafe { ANeuralNetworksExecution_startCompute(run1, &mut run1_end) };

    unsafe { ANeuralNetworksEvent_wait(run1_end) };
    unsafe { ANeuralNetworksEvent_free(run1_end) };
    unsafe { ANeuralNetworksExecution_free(run1) };

    unsafe { ANeuralNetworksCompilation_free(compilation) };
    unsafe { ANeuralNetworksModel_free(model) };

    assert_eq!(out, [2f32, 4., 6., 8., 10., 12., 14., 16., 18.]);
}
