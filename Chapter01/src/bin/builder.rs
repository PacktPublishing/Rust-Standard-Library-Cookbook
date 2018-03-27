fn main() {
    // We can easily create different configurations
    let normal_burger = BurgerBuilder::new().build();
    let cheese_burger = BurgerBuilder::new().cheese(true).salad(false).build();
    let veggie_bigmac = BurgerBuilder::new().vegetarian(true).patty_count(2).build();

    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }
    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }
    if let Ok(veggie_bigmac) = veggie_bigmac {
        veggie_bigmac.print();
    }

    // Our builder can perform a check for
    // invalid configurations
    let invalid_burger = BurgerBuilder::new().vegetarian(true).bacon(true).build();
    if let Err(error) = invalid_burger {
        println!("Failed to print burger: {}", error);
    }

    // If we omit the last step, we can reuse our builder
    let cheese_burger_builder = BurgerBuilder::new().cheese(true);
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(cheese_burger) = cheese_burger {
            println!("cheese burger number {} is ready!", i);
            cheese_burger.print();
        }
    }
}

struct Burger {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}
impl Burger {
    // This method is just here for illustrative purposes
    fn print(&self) {
        let pretty_patties = if self.patty_count == 1 {
            "patty"
        } else {
            "patties"
        };
        let pretty_bool = |val| if val { "" } else { "no " };
        let pretty_vegetarian = if self.vegetarian { "vegetarian " } else { "" };
        println!(
            "This is a {}burger with {} {}, {}cheese, {}bacon and {}salad",
            pretty_vegetarian,
            self.patty_count,
            pretty_patties,
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
    // in the constructor, we can specify
    // the standard values
    fn new() -> Self {
        BurgerBuilder {
            patty_count: 1,
            vegetarian: false,
            cheese: false,
            bacon: false,
            salad: true,
        }
    }

    // Now we have to define a method for every
    // configurable value
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

    // The final method actually constructs our object
    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            patty_count: self.patty_count,
            vegetarian: self.vegetarian,
            cheese: self.cheese,
            bacon: self.bacon,
            salad: self.salad,
        };
        // Check for invalid configuration
        if burger.vegetarian && burger.bacon {
            Err("Sorry, but we don't server vegetarian bacon yet".to_string())
        } else {
            Ok(burger)
        }
    }
}
