pub enum InputType {
    Input,
    Example,
    Example2,
}

pub fn get_input_lines(input_type: InputType) -> Vec<&'static str>{
    let input: &str;
    match input_type {
        InputType::Example => input = include_str!("../example.txt"),
        InputType::Example2 => input = include_str!("../example2.txt"),
        InputType::Input => input = include_str!("../input.txt"),
    }
    let mut data: Vec<&str> = Vec::new();
    for line in input.lines() {
        data.push(line)
    }
    data
}