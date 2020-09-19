//Structs with annotated lifetimes.
struct Rectangle<'a> {
    pub color: &'a String,
    pub height: u32,
    pub width: u32,
}

struct PrivateRectangle<'a> {
    color: &'a String,
    height: u32,
    width: u32,
}

//Method declaratiions for structs.
impl<'a> Rectangle<'a>{
}

impl<'a> PrivateRectangle<'a>{
    //We have a new function here because unlike the previous one this one has private fields.
    fn new(width : u32, height : u32, color : &'a String) -> Self{
        return Self{
            width: width,
            height: height,
            color: color
        }
    } 
}

//Define the Print trait.
trait Print<'a>{

    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn color(&self) -> &'a String;

    fn print(&self) {
        println!("Dimensions: {}w x {}h", self.width(), self.height());
        println!("Color is {}.", self.color());
    }
}

//Implement the Print trait for PrivateRectangle.
impl<'a> Print<'a> for Rectangle<'a> {
    fn width(&self) -> u32{
        return self.width;
    }

    fn height(&self) -> u32{
        return self.height;
    }

    fn color(&self) -> &'a String{
        return self.color;
    }
}

//Implement the Print trait for PrivateRectangle.
impl<'a> Print<'a> for PrivateRectangle<'a> {
    fn width(&self) -> u32{
        return self.width;
    }

    fn height(&self) -> u32{
        return self.height;
    }

    fn color(&self) -> &'a String{
        return self.color;
    }
}

//The 'a are lifetime specifications saying that the string references must live as long as the struct they belong to.


fn main() {
    println!("\nWe will create a white rectangle struct.");
    let rect = Rectangle {
        color: &"white".to_string(),
        height: 1893,
        width: 1933
    };

    println!("\nNow we will create a private red rectangle struct using the new method as our previous instantiation method won't work because the fields are private.");
    println!("\nBut first we have to put a string on the heap to reference.");
    let color = "red".to_string();
    let private_rect = PrivateRectangle::new(2021943, 465, &color);

    println!("\nNow we will print out the values of both using the print Trait common to both.");
    rect.print();
    private_rect.print();


}