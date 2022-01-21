#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {

    println!("Inputs length : {:?}" , inputs.len());

    if inputs.len() == 0 {
        return None;
    }

    match inputs.len() {
        0 => None,
        1 => {
            return match inputs[0] {
                CalculatorInput::Value(value) => {
                    Some(value)
                }
                _ => {
                    None
                }
            };
        }
        2 => None,
        _ => {
            let temp_value1 = 0;
            let temp_value2 = 0;
            let temp_calculator_vec = Vec::new();
            let result = 0;

            match inputs[0] {
                CalculatorInput::Value(value) => {
                    {}
                }
                _ => {
                    None
                }
            }

            match inputs.pop() {
                CalculatorInput::Value(value) => {
                    temp_value1 = value;
                }
                _ => {
                    None
                }
            }
        },
    }
}
