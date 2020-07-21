//similar to tuple
// can be imagined as custom data type with user defined fields
// we can use structs by creating instances of it.

#[derive(Debug)]    //this is only applied to structs, enums and union.

struct Car {       //multiple instance of CAR can be made making it reusable.
    name: String,
    model: String,
    colour: String,
    year: u32,
    status: bool,
}

struct Rectangle {
    number1: u32,
    number2: u32,
}

fn main() {

    let honda = Car {
        name: String::from("city"),
        model: String::from("xzi"),
        colour: String::from("Golden"),
        year: 2007,
        status: true,
    };

    let maruti = Car {
        name: String::from("swift"),
        model: String::from("xv"),
        ..honda     //using remaining values defined in honda
        // colour: honda.colour,
        // year: honda.year,
        // status: honda.status,
    };

    // let car_name = &honda.name;
    // println!("{:?}", honda);
    println!("{:?}", maruti);


    //tuples as structs

    let tuple_one = (1,2,3,4);
    let result = tuple_one.0 + tuple_one.1;
    println!("{}", result);


    //Rectangle struct example
    let rec_fig = Rectangle {
        number1: 1,
        number2: 5,
    };

    println!("print: {}",multiply(&rec_fig));

}

fn multiply(fig: &Rectangle) -> u32 {
    fig.number1 * fig.number2
}