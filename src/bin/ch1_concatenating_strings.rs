fn main() {
    by_moving();
    by_cloning();
    by_mutating();
}

fn by_moving() {
    let hello = "hello".to_string();
    let word = "word";

    let hello_word = hello + " " + word;

    println!("{}", hello_word);
}

fn by_cloning() {
    let hello = "hello".to_string();
    let word = "word";

    let hello_word = hello.clone() + " " + word;

    println!("{} | {}", hello_word, hello);
}

fn by_mutating() {
    let mut hello = "hello ".to_string();
    let word = "word";

    hello.push_str(word);

    println!("{}", hello);
}
