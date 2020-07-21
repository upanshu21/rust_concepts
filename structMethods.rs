// //they are defined within the context of a struct

struct Person {

    first_name: String,
    last_name: String,
}

impl Person {    //methods related to struct Person are defined in impl 

    fn full_name(&self) {
        self.first_name + &self.last_name;
    }
}

fn main() {

    let person = Person {
        first_name: String::from("Upanshu"),
        last_name: String::from("Chaudhary"),
    };

    println!("The full name of the person is {:?}",
    person.full_name()
);

}
