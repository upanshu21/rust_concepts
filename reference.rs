fn main() {

    let dummy = String::from("hello");

    let length = calculate_length(&dummy);

    //---------mutable references---------

    let mut dummy_two = String::from("What's");

    add_string(&mut dummy_two);
    println!("length of dummy is: {}", length);

    
    //let mut result = add_string(&mut dummy_two);   -----not printing-----
    //println!("length of dummy is: {}", result);


    //------mutable reference restriction--------

    let mut restriction = String::from("Hi");

    let var1 = &mut restriction;
    //let var2 = &mut restriction;      this will not work because we can only have one mut ref in scope.

    println!("{}",var1);
}

fn calculate_length(s: &String) -> usize {     //passing reference of the string

    s.len()
}

fn add_string(s : &mut String) {

    s.push_str(", up?");
}