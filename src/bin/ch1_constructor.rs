use std::borrow::Cow;

fn main() {
    let name_length = NameLength::new("Kate");
    name_length.print();
}

struct NameLength<'a> {
    name: Cow<'a, str>,
    length: usize,
}

impl<'a> NameLength<'a> {
    fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let name: Cow<'a, str> = name.into();

        NameLength {
            length: name.len(),
            name,
        }
    }

    fn print(&self) {
        println!("The name is {} characters {} long", self.name, self.length)
    }
}
