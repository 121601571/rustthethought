mod aaa;
use aaa::print_aaa;
fn main() {
    println!("Hello, world!");
    unbor();
    bor();
    let c = aaa::simpeladd(1,2);
    print!("{}", c);
    // add closure here..
    let f1 = aaa::simpeladd;
    let res = f1(3,4);
    print!("{}...", res);

}
fn unbor(){

    let v1 = vec![1,2,3,4,5];
    for i in v1.iter(){
        println!("{}", *i);
    }
    // value not borrowed..
    println!("{:?}", v1);

}
fn bor(){

    let v1 = vec![1,2,3,4,5];
    for i in v1{
        println!("{}", i);
    }
    // value  borrowed..
    //println!("{:?}", v1);

}