fn main() {
    //Test Selections: if, if-else, nested if-else
    let i1=1;
    let i2=2;
    let i3=3;
    let i4=4;
    let i5=5;
    let i6=6;

    //if
    if i4 > i1 {
        println!("i4 >  i1");
    }

    //if-else
    if i5 < i2 && i3 >= i2 {
        println!("i5 <  i2 && i3 >= i2");
    }else{
        println!("i5 >= i2 || i3 <  i2");
    }

    //nested if-else
    if i1 != i2 {
        println!("i1 != i2");
     }
     else {
        if i4 == i5 || i5 != i6 {
           println!("(i1 == i2 && i4 == i5) || (i5 != i6)");
        }
     }
}