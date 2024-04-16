#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog {
    name: String
}

#[derive(Debug)]
enum Animals {
    Manchhe(Person),
    Kukur(Dog)
}

pub fn main() {
    let animals = [
        Animals::Kukur(Dog { name: "Dalle".to_owned() }),
        Animals::Manchhe(Person { name: "Robo".to_owned() })
    ];
    let creatures = (Person{name: "Uzzal".to_owned()}, Dog{name: "Dalle".to_owned()});
    println!("{:?}", animals);
    println!("{:?}", creatures.1);
}
