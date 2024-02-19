fn main() {
    //! s will be droped after the function call
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    //! s1 will be droped after the function call because it is moved
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 = {}, s2 = {}", s1, s2);

    //! s1 will not be droped after the function call becays it is cloned
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    //! s will be droped after the function call but not x because it's a copy
    // let s = String::from("hello");
    // takes_ownership(s);
    // let x = 5;
    // makes_copy(x);

    //! s1 will be droped after the function call because it is moved
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);

    //! Rust does let us return multiple values using a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}