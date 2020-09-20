fn main() {
    println!("Mutability is a key concept in Rust.");
    println!("A variable is a constant unless explicitly declared differently.");
    let x = 5;
    println!("We just created a variable but we cannot assign a new value to it. x = {}", x);
    println!("So we will create a new variable.");
    let mut x = 10;
    println!("Here is our new mutable variable, x = {}", x);
    x = 15;
    println!("And now we just assigned a new value to x; x = {}", x);

    println!("---\nSo what happened? Did we just override mutability? No. Rust has another feature called 'shadowing' which lets you override old variables with new ones. But it doesn't modify the old one. It just hides or sends it to the background where it can't be accessed without a different name or memory reference. A mutable variable on the other hand can have new values assigned to it on the fly.");
}