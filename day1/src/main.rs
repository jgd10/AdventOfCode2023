fn import_input() -> &'static str{
    let data: &str;
    data = include_str!("../input.txt");
    return data
}


fn part1() {
    let data: &'static str = import_input();
    println!("{}", data);
}


fn main() {
    part1();
}
