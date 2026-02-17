pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points_vec: Vec<(usize, usize)> = Vec::new();

    if (*input).len() == 0 {
        return saddle_points_vec;
    } else {
        if (*input)[0].len() == 0 {
            return saddle_points_vec;
        }
    }

    println!("Rows number is {} .", (*input).len());
    println!("Colunms number is {} .", (*input)[0].len());

    //Traverse tables
    let mut row_iter = (*input).iter();
    let mut colum_iter;
    loop {
        if let Some(x) = row_iter.next() {
            colum_iter = x.iter();
            loop {
                if let Some(y) = colum_iter.next() {
                    print!(" [{}] ", y);
                } else {
                    print!("\n");
                    break;
                }
            }
        } else {
            break;
        }
    }

    let mut row_iter = (*input).iter();
    let mut row_index = 0;
    let mut colum_iter;
    loop {
        if let Some(table_row_iterm) = row_iter.next() {
            println!("Row index[{}] value is {:?}", row_index, table_row_iterm);

            //Max value of a row
            colum_iter = table_row_iterm.iter();
            let max_in_row = colum_iter.max().unwrap();
            println!("Row index[{}] max element is {} .", row_index, max_in_row);

            //Position
            //If find a value position , iter set this position's next element as new index 0
            let mut pre_position = -1;
            colum_iter = table_row_iterm.iter(); //back to the beginning
            loop {
                if let Some(next_position) = colum_iter.position(|&x| x == *max_in_row) {
                    let index = pre_position + 1 + (next_position as i32);
                    pre_position = index as i32;
                    println!("{} index is {} .", *max_in_row, index);

                    //Check the colum value if is min
                    let mut if_min: bool = true;
                    for i in (*input).iter() {
                        if i.iter().nth(index as usize).unwrap() < max_in_row {
                            println!(
                                "[{}][{}]:{} is not a saddle point !",
                                row_index, index, max_in_row
                            );
                            if_min = false;
                            break;
                        }
                    }
                    if if_min {
                        println!(
                            "[{}][{}]:{} is a saddle point .",
                            row_index, index, max_in_row
                        );
                        saddle_points_vec.push((row_index, index as usize));
                    }
                } else {
                    break;
                }
            }

            row_index += 1;
        } else {
            break;
        }
    }

    return saddle_points_vec;
}
