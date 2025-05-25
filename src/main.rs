
fn main() {
    let ans: u32 = sum(1, 2); // Correct: positional arguments
    println!("{}", ans);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}
// comment
fn main(){

}

fn main(){
    let s: i32 = -92;
    println!("{}",s);
}
fn main(){
    let is_male = true;
    let is_above_18 = false;
    
    if is_male {
        println!("you are a male");
    } else {
        println!("you are not a male");
    }
    if is_male && is_above_18 {
        println!("you are a legal maleeee");

    }
}

// conditions and loops are same no need 
// trying immutability different form type script 

fn main(){
    let mut s: i32 = -92;
    println!("{}",s);
    s = 100;
    println!("{}",s);
    // s = "hello"; // this will not work
    // println!("{}",s);
    let mut a = 10;
    let mut b = 20;
    let mut c = 30;
    let a = 90;
    if a > b {
        println!("a is greater than b");
    } else {
        println!("b is greater than a");
    }
    let c = 100;
    if a > b && a > c {
        println!("a is the greatest");
    } else if b > a && b > c {
        println!("b is the greatest");
    } else {
        println!("c is the greatest");
    } 
}


// owner ship and heap sounds interesting

fn main(){
    let str = String::from("kugarabachi");
    let len = get_length(str.);
    println!("{}", len); //println! means new line 
    print!("{}",str);     //print! means just print no new line     
}

fn get_length(str:String) -> usize {
    return str.len()
}

// to fix this either use clone which will work but no point as it diminishes why we use rust or the correct way transfer back ownershi[]
fn main(){
    let str = String::from("kugarabachi");
    let (str, len) = get_length(str);
    println!("{} {}", str ,len); 
}
fn get_length(str: String) -> (String, usize) {
    let len = str.len();  //tranfered back ownership
    return (str, len);
}

// borrowing 

fn main() {
    let str = String::from("kugarabachi");
    let len = get_length(&str);
    println!("{} {}", str, len);
}

fn get_length(str: &String) -> usize {
    let len = str.len();
    return len
}// much prefer this over tranfeering ownership


fn main() {
    let str = String::from("Harkirat");
    let ref1 = &str;
    let ref2 = &str;
     let ref3 = &str;
    println!("{} {} {}", ref1, ref2, ref3);
}


fn main() {
    let mut str = String::from("Harkirat");
    let ref1 = &mut str;
//    let ref2 = &str;
    
    println!(" {}", ref1);
}

fn main() {
    let mut str = String::from("Harkirat");
  //  let ref1 = &mut str;
    let ref2 = &str;
    
    println!(" {}", ref2);
}


//assignment 

fn calculate_length(s: &String) -> usize {
    s.len()cargo run
}

fn main() {
    let sttring = String::from("I'm a genius");
    let length = calculate_length(&sttring);

    println!("{},{}", sttring, length);
}

// assignement 2 


fn append_text(s:&mut String){
    s.push_str("Your Wish My Command");
}

fn main(){
    let mut sttring = String::from(" MY Queen ");
    append_text(&mut sttring );
    println!("{}", sttring)

}

// structs and enums are quite simple 

// writting a fn to take shape and print area and perimeter 


// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side_length) =>length * length,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
  
    let circle = Shape::Circle(9.0);
    let square = Shape::Square(87.0);
    let rectangle = Shape::Rectangle(2.0, 7.0);

    
    println!("Area circle: {}", calculate_area(circle));
    println!("Area square: {}", calculate_area(square));
    println!("Area rectangle: {}", calculate_area(rectangle));
}





use std::fs;

fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read : {}", file_content);
        },
        Err(error) => {
            println!("Failed to read file: {}", error);
        }
    }
}


use std::ops::Add;
fn main (){
    println!("{}" , sum(1, 2));
}
fn sum<T: Add<Output = T>>(a:T , b:T) -> T {
    return a + b 
}












