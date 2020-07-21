//following code will not run. This is for an understanding of enums.
//todo: modify this into a code and maintain enum understanding.

//enums are used when a custom data type can have variations.
//for eg: a data type IP adress can have IPV4 and IPV6 as variant.
//enums can be used with structs.
//enums can be used in place of structs

// -------enums used with structs

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};


//enums used in place of structs

enum IpAddrr {
    V4(String),   //V4(u8, u8, u8, u8),
    V6(String),   
}


let home = IpAddr::V4(String::from("127.0.0.1"));   //let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));


// we can put anytype of data in an enum variant like string, numeric type or struct 

//enum data type variant

enum Message {
    
    Quit,    //this has no data assiciated with it
    Move { x: u32, y: u32},   //anonymous struct || we do not need to use struct keyword inside enum
    Write(String),     //includes a single string
    ChangeColor(i32,i32,i32),      //includes three i32 values
}