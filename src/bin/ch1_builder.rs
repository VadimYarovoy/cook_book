fn main() {
    let normal_burger = BurgerBuilder::new().build();
    normal_burger.unwrap().print();

    let full_burger = BurgerBuilder::new()
        .patty_count(2)
        .bacon(true)
        .cheese(true)
        .salad(true)
        .build();
    full_burger.unwrap().print();

    let vegetarian_burger = BurgerBuilder::new().vegetarian(true).build();
    vegetarian_burger.unwrap().print();
}

struct Burger {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl Burger {
    fn print(&self) {
        let pretty_paties = if self.patty_count == 1 {
            "patty"
        } else {
            "patties"
        };

        let pretty_bool = |val| if val { "" } else { "no " };
        let pretty_vegetarian = if self.vegetarian { "vegetarian" } else { "" };

        println!(
            "This is a {}burger with {} {}, {}cheese, {}bacon and {}salad.",
            pretty_vegetarian,
            self.patty_count,
            pretty_paties,
            pretty_bool(self.cheese),
            pretty_bool(self.bacon),
            pretty_bool(self.salad)
        )
    }
}

struct BurgerBuilder {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl BurgerBuilder {
    fn new() -> Self {
        BurgerBuilder {
            patty_count: 1,
            vegetarian: false,
            cheese: false,
            bacon: false,
            salad: true,
        }
    }

    fn patty_count(mut self, val: i32) -> Self {
        self.patty_count = val;
        self
    }

    fn vegetarian(mut self, val: bool) -> Self {
        self.vegetarian = val;
        self
    }

    fn cheese(mut self, val: bool) -> Self {
        self.cheese = val;
        self
    }

    fn bacon(mut self, val: bool) -> Self {
        self.bacon = val;
        self
    }

    fn salad(mut self, val: bool) -> Self {
        self.salad = val;
        self
    }

    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            patty_count: self.patty_count,
            vegetarian: self.vegetarian,
            cheese: self.cheese,
            bacon: self.bacon,
            salad: self.salad,
        };

        if burger.vegetarian && burger.bacon {
            Err("Sorry, but we don't server vegetarian bacon yet".to_string())
        } else {
            Ok(burger)
        }
    }
}
