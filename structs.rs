//similar to tuple
// can be imagined as custom data type with user defined fields
// we can use structs by creating instances of it.

#[derive(Debug)]

struct Car {       //multiple instance of CAR can be made making it reusable.

    name: String,
    model: String,
    colour: String,
    year: u32,
    status: bool,

}

fn main() {

    let honda = Car {
        name: String::from("city"),
        model: String::from("xzi"),
        colour: String::from("Golden"),
        year: 2007,
        status: true,
    };

    // let car_name = &honda.name;
    println!("{:?}", honda);

}