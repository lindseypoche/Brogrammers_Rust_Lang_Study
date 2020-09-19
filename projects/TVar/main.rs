fn main() {
    let boolean : bool = true;

    let uvar1 : u8 = 1;
    let uvar2 : u16 = 1;
    let uvar3 : u32 = 1;
    let uvar4 : usize = 1;

    let ivar1 : i8 = -1;
    let ivar2 : i16 = -1;
    let ivar3 : i32 = -1;
    let ivar4 : isize = -1;

    let fvar1 : f32 = 1.8675309;

    let string_literal : &str = "Hello";
    let string_on_heap : String = "I can haz cheezburger?".to_string();

    let option_some : Option<()> = Some(());
    let option_none : Option<()> = None;
    let result_ok : Result<(),()> = Ok(());
    let result_err : Result<(),()> = Err(());

    println!("Boolean: {}", boolean);
    println!("Unsigned var 1 through 4: {} {} {} {}", uvar1, uvar2, uvar3, uvar4);
    println!("Signed var 1 through 4: {} {} {} {}", ivar1, ivar2, ivar3, ivar4);
    println!("Float var 1: {}", fvar1);
    println!("String literal: {}", string_literal);
    println!("String on the heap: {}", string_on_heap);
    println!("Option with something: {}", option_some.is_some());
    println!("Option without something: {}" ,option_none.is_none());
    println!("Result that is ok: {}", result_ok.is_ok());
    println!("Result that is err: {}", result_err.is_err());
}