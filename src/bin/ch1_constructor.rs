fn main() {
    let name_length = NameLength::new("Kate");
    name_length.print();
}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    fn new(name: &str) -> Self {
        NameLength {
            length: name.len(),
            name: name.to_string(),
        }
    }

    fn print(&self) {
        println!("The name is {} characters {} long", self.name, self.length)
    }
}
