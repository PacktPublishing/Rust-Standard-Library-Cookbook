#[macro_use]
extern crate bitflags;

bitflags! {
    struct Spices: u32 {
        const SALT       = 0b0000_0001;
        const PEPPER     = 0b0000_0010;
        const CHILI      = 0b0000_0100;
        const SAFFRON    = 0b0000_1000;
        const ALL        = Self::SALT.bits
                         | Self::PEPPER.bits
                         | Self::CHILI.bits
                         | Self::SAFFRON.bits;
    }
}

impl Spices {
    // Implementing a "clear" method can be useful
    pub fn clear(&mut self) -> &mut Self {
        self.bits = 0;
        self
    }
}

fn main() {
    let classic = Spices::SALT | Spices::PEPPER;
    let spicy = Spices::PEPPER | Spices::CHILI;
    // Bit fields can nicely be printed
    println!("Classic: {:?}", classic);
    println!("Bits: {:08b}", classic.bits());
    println!("Spicy: {:?}", spicy);
    println!("Bits: {:08b}", spicy.bits());

    println!();

    // Use set operations
    println!("Union: {:?}", classic | spicy);
    println!("Intersection: {:?}", classic & spicy);
    println!("Difference: {:?}", classic - spicy);
    println!("Complement: {:?}", !classic);

    // Interact with flags in a bit field
    let mut custom = classic | spicy;
    println!("Custom spice mix: {:?}", custom);
    custom.insert(Spices::SAFFRON);
    // Note that ALL is now also contained in the bit field
    println!("Custom spice after adding saffron: {:?}", custom);
    custom.toggle(Spices::CHILI);
    println!("Custom spice after toggling chili: {:?}", custom);
    custom.remove(Spices::SALT);
    println!("Custom spice after removing salt: {:?}", custom);

    // This could be user input
    let wants_salt = true;
    custom.set(Spices::SALT, wants_salt);
    if custom.contains(Spices::SALT) {
        println!("I hope I didn't put too much salt in it");
    }

    // Read flags from raw bits
    let bits = 0b0000_1101;
    if let Some(from_bits) = Spices::from_bits(bits) {
        println!("The bits {:08b} represent the flags {:?}", bits, from_bits);
    }

    custom.clear();
    println!("Custom spice mix after clearing: {:?}", custom);
}
