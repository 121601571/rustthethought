fn main() {
    println!("Hello, world!");
    unbor();
    bor();
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