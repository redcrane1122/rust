fn add_fn (a: i32, b: i32) -> i32 {
    a + b
}

fn maybe_num() -> Option<i32> {
    let num = 1;
    match num {
        1 => Some(num),
        _ => None,
    }
}

fn maybe_word() -> Option<String> {
    return Some("A");
}

fn main() {
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    let add = |a, b| a + b;
    let sum = add (1,1);

    let plus_one = match maybe_num() {
        Some(num) => Some(num + 1),
        None => None,
    };

    let plus_one = maybe_num().map(|num| num + 1);

    let word_length = maybe_word()
        .map(|word| word.len())
        .map(|len| len * 2);
}
