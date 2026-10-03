trait Animal {
    fn make_sound(&self);
}

struct Dog;

fn main() {
    let dog = Dog;
    dog.make_sound();//error
}
