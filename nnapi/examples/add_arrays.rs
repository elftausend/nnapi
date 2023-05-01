use nnapi::{Model, Operand};
use nnapi_sys::{OperandCode, OperationCode};

fn main() -> nnapi::Result<()> {
    let tensor9x_type = Operand::tensor(OperandCode::ANEURALNETWORKS_TENSOR_FLOAT32, &[9], 0., 0);

    let mut model = Model::from_operands([
        tensor9x_type,
        tensor9x_type,
        Operand::activation(),
        tensor9x_type,
    ])?;

    model.set_activation_operand_value(2)?;
    model.add_operation(OperationCode::ANEURALNETWORKS_ADD, &[0, 1, 2], &[3])?;
    model.identify_inputs_and_outputs(&[0, 1], &[3])?;

    model.finish()?;

    let mut compilation = model.compile()?;
    compilation.finish()?;
    let mut execution = compilation.create_execution()?;

    // mind datatype: by default, it's f64, but we need f32
    execution.set_input(0, &[1f32; 9])?;
    execution.set_input(1, &[2f32; 9])?;

    let mut output = [0f32; 9];
    execution.set_output(0, &mut output)?;

    let mut end_event = execution.compute()?;
    end_event.wait()?;

    assert_eq!(output, [3f32; 9]);

    Ok(())
}
