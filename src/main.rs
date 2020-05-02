// #[allow(unused_imports)]
use std::time::Duration;
use futures::executor::block_on;
// #[allow(unused_imports)]
use std::thread;


fn main() {
    // calling async main thread and blocking it with the
    // block_on() executor.
    println!("IMPLIMENTING THROUGH ASYNC");
    block_on(async_main());
    // calling threaded function
    println!("\nIMPLIMENTING THROUGH THREAD");
    let learning = thread::spawn(||{
        for i in 0..5{
            println!("learning {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let singing = thread::spawn(||{
        for i in 0..5 {
            println!("Singing {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Dancing {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    learning.join().unwrap();
    singing.join().unwrap();
}

async fn async_main() {
    // calling dancing async function and awaiting it to
    // return a future
    dance().await;
    // calling learn_sing async function and awaiting it to
    // return a future.
    learn_sing().await;
    // let f1 = learn_sing();
    // let f2 = dance();
    // futures::join!(f2, f1);
}

async fn learn_sing() {
    let song = learn().await;
    sing(song).await;
}

async fn learn()-> String{
    println!("Learning");
    "Learning done".to_string()
}

async fn sing(song: String){
    if song==String::from("Learning done"){
        println!("Start Singing");}
    else {
        println!("Please learn first");
    }
}

async fn dance(){
    println!("Dancing");
}