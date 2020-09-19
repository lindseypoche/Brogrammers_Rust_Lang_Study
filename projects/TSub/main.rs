fn main() {
    //Test function
    simple();
    println!("{}",complicated(4));
}

fn simple(){
    println!("Simple function.");
}

fn complicated(number : u32) -> String{
    if number > 0 {
        return format!("{} is greater than zero.",number)
    } else {
        return format!("{} is less than or equal to zero.",number)
    }
}