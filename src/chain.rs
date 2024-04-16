#[derive(Debug)]
struct Dog {
    name: String,
    steps: u8
}

impl Dog {
    fn walk(&mut self) -> &mut Self {
        println!("{} walks.", self.name);
        self.steps += 2;
        self
    }
    fn run(&mut self) -> &mut Self {
        println!("{} runs.", self.name);
        self.steps += 6;
        self
    }
}

pub fn main() {
    let mut dalle = Dog { name: "Dalle".to_string(), steps: 0 };
    dalle.walk().run().walk().run().walk().walk().walk().run().run();
    println!("{:?}", dalle);
}
