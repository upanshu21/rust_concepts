fn main() {

    //example1
    let dummy = String::from("hello");

    let slice = &dummy[0..2];

    println!("{}", slice);


    //method 2

    let dummy_two = String::from("hello world");
    let length = dummy_two.len();
    let slice_two = &dummy_two[..length];

    println!("{}", slice_two);


    //example 

    let a = first_word(&dummy_two);
    println!("{}", a);


    //slicing array

    // let array = [1,2,3,4,5];     ----------         -------

    // let result = &array[1..3];   ----------   doubt ------

    // println!("{}", result);      ----------          -------

}

//program to find first word
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}