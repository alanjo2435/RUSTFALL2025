enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        println!("Available fruits in the grocery store:");
        for fruit in &self.fruit {
            match fruit {
                Fruit::Apple(_) => println!("- Apple"),
                Fruit::Banana(_) => println!("- Banana"),
                Fruit::Tomato(_) => println!("- Tomato"),
            }
        }
    }

    fn tell_me_joke(fruit: &Fruit) {
        match fruit {
            Fruit::Apple(_) => println!("Why did the apple cry? Because it lost its core of confidence."),
            Fruit::Banana(_) => println!("Why did the banana go to the doctor? Because it wasn’t peeling well."),
            Fruit::Tomato(_) => println!("Why did the tomato turn red? Because it saw the salad dressing."),
        }
    }
}

fn main() {
    let a = "An apple a day keeps the doctor away.".to_string();
    let b = "A banana boosts energy in a peel.".to_string();
    let t = "A tomato a day keeps the sunburn away.".to_string();

    let fruits = vec![
        Fruit::Banana(b),
        Fruit::Apple(a),
        Fruit::Tomato(t),
    ];

    let grocery_store = Inventory { fruit: fruits };

    grocery_store.available_fruits();

    println!("\nFruit Jokes:");
    for f in &grocery_store.fruit {
        Inventory::tell_me_joke(f);
    }
}
