fn main() {

    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    //another way to access tuple elements is

    let tuple_two : (f64 , i32, i32) = (99.5 , 5, 7);    //remember to use snake case naming convention otherwise the compiler will throw error.

    let element_one = tuple_two.0;
    let element_two = tuple_two.1;
    let element_three = tuple_two.2;

    println!("number at 0 is: {}", element_one);
    println!("number at 1 is: {}", element_two);
    println!("number at 2 is: {}", element_three);

}