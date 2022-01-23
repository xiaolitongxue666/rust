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
                    return None;
                }
            };
        },

        2 => None,

        _ => {
            let mut temp_value1 = 0;
            let mut temp_value2 = 0;
            let mut temp_index = 0;
            let mut temp_step_count = 0;
            let mut result_vec = Vec::new();

            println!("Inputs : {:?}" , inputs.clone());

            for inputs_vec_value in inputs.iter() {
                match inputs_vec_value {
                    CalculatorInput::Value(value) => {
                        temp_index +=1;
                        if temp_step_count == 0 {
                            temp_value1 = *value;
                            temp_step_count += 1;
                        }
                        else if temp_step_count == 1 {
                            temp_value2 = *value;
                        }

                        println!("Step[{}] Value1[{}] Value2[{}]" , temp_step_count.clone(), temp_value1.clone(), temp_value2.clone());
                    },
                    CalculatorInput::Add => {
                        temp_index +=1;
                        if temp_step_count == 1 {
                            println!("{} + {}", temp_value1.clone(), temp_value2.clone());
                            result_vec.push(temp_value1 + temp_value2);
                            println!("Result : {:?}", result_vec.clone());
                            temp_step_count = 0;
                        }
                        else {
                            if temp_index < inputs.len()
                            {
                                return None;
                            }
                        }
                    },
                    CalculatorInput::Subtract => {
                        temp_index +=1;
                        if temp_step_count == 1 {
                            println!("{} - {}", temp_value1.clone(), temp_value2.clone());
                            result_vec.push(temp_value1 - temp_value2);
                            println!("Result : {:?}", result_vec.clone());
                            temp_step_count = 0;
                        } else {
                            if temp_index < inputs.len()
                            {
                                return None;
                            }
                        }
                    },
                    CalculatorInput::Multiply => {
                        temp_index +=1;
                        if temp_step_count == 1 {
                            result_vec.push(temp_value1 * temp_value2);
                            temp_step_count = 0;
                        } else {
                            if temp_index < inputs.len()
                            {
                                return None;
                            }
                        }
                    },
                    CalculatorInput::Divide => {
                        temp_index +=1;
                        if temp_step_count == 1 {
                            println!("{} / {}", temp_value1.clone(), temp_value2.clone());
                            result_vec.push(temp_value1 / temp_value2);
                            println!("Result : {:?}", result_vec.clone());
                            temp_step_count = 0;
                        } else {
                            if temp_index < inputs.len()
                            {
                                return None;
                            }
                        }
                    },
                }
            }



            return if inputs.len() > 3 {

                println!("Do one more calculate");
                match inputs[temp_index - 1] {
                    CalculatorInput::Add => {
                        temp_value1 = result_vec[0] + result_vec[1];
                    },
                    CalculatorInput::Subtract => {
                        temp_value1 = result_vec[0] - result_vec[1];
                    },
                    CalculatorInput::Multiply => {
                        temp_value1 = result_vec[0] * result_vec[1];
                    },
                    CalculatorInput::Divide => {
                        temp_value1 = result_vec[0] / result_vec[1];
                    },
                    _ => {
                        return None;
                    }
                }
                Some(temp_value1)
            } else {
                Some(result_vec[0])
            }
        },
    }
}
