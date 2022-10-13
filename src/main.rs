
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
fn ren3<T:Test>(t1:T){

}
fn ren1(t1:impl Test,t2:impl Test){

}
fn ren2<T:Test>(t1:T,t2:T){

}
fn main() {
    let d = dog{name:String::from("wanwan")};
    d.dushu();
    println!("test81");
}
