use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    // unimplemented!("convert a color into a numerical representation")
    // println!("color as usize is {}" , color as usize);
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    // unimplemented!("convert the value {} into a string representation of color" , value)
    match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")

    ResistorColor::into_enum_iter().collect()
}
