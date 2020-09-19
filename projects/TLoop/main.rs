fn main() {
    println!("Simple for-loop | STOPS ON i < 5");
    for i in 0..5{
        println!("i = {}",i);
    }

    println!("\nIterator example. Loop through an array. | STOPS ON array end");
    let tanks = [
        "Valentine",
        "KV-2",
        "Somua",
        "Cromwell",
        "Sherman",
        "Matilda",
        "Tiger"
    ];
    for tank in tanks.iter() {
        println!("tank = {}",tank);
    }

    println!("\nTuple Iterator example. Loop through a tuple array. | STOPS ON array end");
    let tanks_and_nationalities = [
        ("Valentine", "British"),
        ("KV-2", "Soviet"),
        ("Somua", "French"),
        ("Cromwell", "British"),
        ("Sherman", "American"),
        ("Matilda", "British"),
        ("Tiger", "German")
    ];

    for (tank, nationality) in tanks_and_nationalities.iter() {
        println!("The {} was a {} tank.",tank, nationality);
    }

    println!("\nWhile loop | STOPS on i < 5");
    let mut i = 0;
    while(i<5){
        println!("i = {}",i);
        i+=1;
    }

    println!("\nUnbreaking loop must use break to exit. | STOPS ON i < 5");
    let mut i = 0;
    loop{
        println!("i = {}",i);
        i+=1;
        if i == 5{
            break;
        }
    }
}