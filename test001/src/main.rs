mod aaa;
use aaa::print_aaa;
use std::thread;
use std::sync::mpsc;
struct Person{
    name:String,
    age:i8,
    address:String,
}

impl Person{
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_age(&self) -> &i8 {
        return &self.age;
    }
}

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

    let a1 = ttest(1,2);
    println!("{}", a1);

    let a = Person{
        name:"aaa".to_string(),
        age:10,
        address: "".to_string()
    };
    println!("{}", a.get_name());
    a.mytest();

    //closure
    let num1 = "a".to_string();
    let num2 = "b".to_string();
    let fn1 =   |num1, num2| {
      println!("{},{}", num1 , num2);
    };
    fn1(num1, num2);
   // println!("{},{}", num1, num2);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

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
fn ttest<T> (a: T, b:T) ->T{
    return a;
}

pub trait mytest {
    fn mytest(&self){
        panic!()
    }
}

impl mytest for Person{
    fn mytest(&self){
            println!("my trait..");
    }
}