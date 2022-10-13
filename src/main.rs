
use std::fmt::Display;
trait Test {
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
impl Test for dog{
    // fn dushu(&self){
    //     println!("wwwwww");
    // }
}
fn ren(t: impl Test ){
    println!("{:?}",t.dushu());

}
fn main() {
    let d = dog{name:String::from("wanwan")};
    d.dushu();
    println!("test81");
}
