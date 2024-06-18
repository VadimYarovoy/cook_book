fn main() {
    let colour = "red";
    let favourite = format!("My favourite colour is {}", colour);
    println!("{}", favourite);

    let colour_one = "red";
    let colour_two = "blue";

    let favourites = format!("My favourite colours are {} and {}", colour_one, colour_two);
    println!("{}", favourites);

    let favourite_num = format!("My favourite number is {}", 15);
    println!("{}", favourite_num);

    let duck_duck_goose = format!("{0}-{0}-{1}", "duck", "goose");
    println!("{}", duck_duck_goose);

    let introduction = format!(
        "My name is {name}, {name} {surname}",
        name = "John",
        surname = "Doe"
    );

    println!("{}", introduction);
}
