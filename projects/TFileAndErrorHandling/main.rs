use std::fs;
use std::fs::File;
use std::io::Write;

fn main(){
    let filename = "test.txt";
    let mut contents = "";
    //A simple example
    println!("In file {}:", filename);
    match fs::read_to_string(filename) {
        //This is cool because we can get values out of Ok and Err results.
        Ok(contents) => {println!("I found stuff: {}\n", contents);},
        Err(_) => {println!("Snake? Snake!? Snaaaake!! I couldn't read the file.\n");}
    }

    println!("We probably couldn't read the file so we will create one and then try again.");
    let mut file = File::create(filename).expect("This crashed because the file could not be created. This is good because expect is working.");
    //You wouldn't get a nice message if this crashed. Both expect and unwrap remove the Err and Ok, and Some and None at the expense of halting the program on failure.
    file.write_all(b"It's Dangerous To Go Alone! Take This. Rust").unwrap();
    
    let contents = fs::read_to_string(filename).expect("Somehow this still failed."); //Simple error handling.

    println!("I got the text: {}", contents);
}