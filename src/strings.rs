pub fn run(){

    let mut hello = String::from("Hello ");

    //Get length 
    println!("Length : {}",hello.len());

    //Push char 
    hello.push('W');

    //push string
    hello.push_str("orld!");

    println!("{}",hello);


}