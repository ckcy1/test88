trait test {
    fn dushu(&self){
        println!("dus");
    }
}
struct dog{
    name:String,
}
struct cat{
    name:String,
}
impl test for dog{
    // fn dushu(&self){
    //     println!("wwwwww");
    // }
}

fn main() {
    let d = dog{name:String::from("wanwan")};
    d.dushu();
    println!("test81");
}
