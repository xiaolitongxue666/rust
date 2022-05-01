#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

     let first_list_len = _first_list.len();
     let second_list_len = _second_list.len();

    let mut index  = 0;
    let result;
    let mut fist_flag = true;

    println!("{:?}", first_list_len);
    println!("{:?}", second_list_len);

    return if _first_list.len() == _second_list.len() {
        if _first_list == _second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else {
        if _first_list.len() > _second_list.len() {
            if _second_list.len() == 0 {
                Comparison::Superlist
            } else {
                // first_list_len > second_list_len
                if _second_list.iter().all(|x| _first_list.contains(x)) {
                    loop {
                        if (fist_flag == false) && (index > (first_list_len - second_list_len)) {
                            result = Comparison::Unequal;
                            break;
                        } else {
                            if _second_list == &_first_list[(index) .. (index + second_list_len)] {
                                result = Comparison::Superlist;
                                break;
                            } else {
                                index += 1;
                            }
                        }
                    }
                    result
                } else {
                    Comparison::Unequal
                }
            }
        } else {
            if _first_list.len() == 0 {
                Comparison::Sublist
            } else {
                // first_list_len < second_list_len
                if _first_list.iter().all(|x| _second_list.contains(x)) {
                    loop {
                        if (fist_flag == false) && (index > (second_list_len - first_list_len)) {
                            result = Comparison::Unequal;
                            break;
                        } else {
                            fist_flag = false;
                            println!("Test: index {:?}", index);
                            println!("Test: index + first_list_len {:?}", index+first_list_len);

                            if _first_list == &_second_list[(index) .. (index + first_list_len)] {
                                result = Comparison::Sublist;
                                break;
                            } else {
                                index += 1;
                            }
                        }
                    }
                    result
                } else {
                    Comparison::Unequal
                }
            }
        }
    };
}
