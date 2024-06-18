fn main() {
    let pizza = PizzaConfig {
        want_cheese: true,
        number_of_olives: 22,
        special_message: "".to_string(),
        crust_type: CrustType::Thick,
    };

    println!(
        "want_cheese {}, number_of_olives {}, special_message {}, crust_type {:?}  ",
        pizza.want_cheese, pizza.number_of_olives, pizza.special_message, pizza.crust_type
    );

    let pizza = PizzaConfig::default();
    println!("{:?}", pizza);
}

#[derive(Default, Debug)]
struct PizzaConfig {
    want_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType,
}

#[derive(Debug)]
enum CrustType {
    Thin,
    Thick,
}

impl Default for CrustType {
    fn default() -> Self {
        CrustType::Thin
    }
}
