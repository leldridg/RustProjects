struct Person {
    name: String,
}

struct Friendship<'a> {
    person_a: &'a Person,
    person_b: &'a Person,
}

impl<'a> Friendship<'a> {
    fn print(&self) {
        println!("{} is friends with {}", self.person_a.name, self.person_b.name);
    }
}

fn main() {
    let alice: Person = Person {
        name: String::from("Alice"),
    };

    let bob: Person = Person {
        name: String::from("Bob"),
    };

    let alice_bob_friendship: Friendship = Friendship {
        person_a: &alice,
        person_b: &bob,
    };

    alice_bob_friendship.print();
}
