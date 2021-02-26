mod aaa {
    const X: i32 = 10;
    pub fn print_aaa() {
        println!("{}", 25);
    }
}
pub fn print_aaa() {
    println!("{}", 25);
}

pub fn simpeladd(a:i32, b:i32)->i32{
    a+b
}

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

fn main(){

}