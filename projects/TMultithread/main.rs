use std::time::Duration;
use std::thread;

fn main() {

    spawn_new_thread(1);
    spawn_new_thread(2);
    spawn_new_thread(3);

    for i in 0..10{
        spawn_new_thread(i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("Close main.")
}

fn spawn_new_thread(id : i32){
    let _new_thread = thread::spawn(move || {
        let mut i = 0;
        loop {
            i+=1;
            println!("From thread {}, i = {}", id, i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
}