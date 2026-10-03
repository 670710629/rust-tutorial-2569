trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

fn make_sound<T>(animal: T) {
    animal.make_sound(); // Error
}

fn main() {
    let dog = Dog;
    dog.make_sound();
}
