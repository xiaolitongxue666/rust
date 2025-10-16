use std::collections::HashMap;

fn plant_from_char(c: char) -> &'static str {
    match c {
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        'V' => "violets",
        _ => "No such plant",
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let names = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let mut name_to_index = HashMap::new();
    for (index, &name) in names.iter().enumerate() {
        name_to_index.insert(name, index);
    }

    // 获取学生索引
    let student_index = match name_to_index.get(student) {
        Some(&index) => index,
        None => return vec![],
    };

    // 按换行符分割
    let lines: Vec<&str> = diagram.lines().collect();
    let plants_index = student_index * 2;

    // 获取第一行第一个字符和第二个字符
    let first_line_first_char = lines
        .first()
        .and_then(|line| line.chars().nth(plants_index));
    let first_line_second_char = lines
        .first()
        .and_then(|line| line.chars().nth(plants_index + 1));

    // 获取第二行第一个字符和第二个字符
    let second_line_first_char = lines.get(1).and_then(|line| line.chars().nth(plants_index));
    let second_line_second_char = lines
        .get(1)
        .and_then(|line| line.chars().nth(plants_index + 1));

    vec![
        plant_from_char(first_line_first_char.unwrap_or(' ')),
        plant_from_char(first_line_second_char.unwrap_or(' ')),
        plant_from_char(second_line_first_char.unwrap_or(' ')),
        plant_from_char(second_line_second_char.unwrap_or(' ')),
    ]
}
