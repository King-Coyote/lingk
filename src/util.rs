use std::io;

pub fn do_while_input<F>(mut action: F) 
where
    F: FnMut(&str) -> Option<()>
{
    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).unwrap() > 0 {
        if action(&input).is_none() {
            break;
        }
        input.clear();
    }
}