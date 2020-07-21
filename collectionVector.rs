// vectors allow us to store n number of values next to each other in memory.
// can only store values of same type 


fn main() {

// creating a new empty vector

let mut empty_vector: Vec<i32> = Vec::new();   //empty vector requires type annotation

empty_vector.push(1);
empty_vector.push(2);
empty_vector.push(3);
empty_vector.push(4);
empty_vector.push(5);

let v = vec!["blue","black","green"];

for numbers in empty_vector.iter() {
    println!("{}", numbers);
}

for words in v.iter() {
    println!("{}",words)
}


//reading elemets from vector

let first_method = &v[2];

let second_method = v.get(0);

println!("resut from first method  is: {} and result from second method is: {:?}"
, first_method
, second_method);


}


