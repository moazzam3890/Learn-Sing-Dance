#[allow(unused_imports)]
use std::time::Duration;
use futures::executor::block_on;
#[allow(unused_imports)]
use std::thread;


fn main() {
    block_on(async_main());
}

async fn async_main() {
    
    dance().await;
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