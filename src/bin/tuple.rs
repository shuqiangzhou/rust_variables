#[allow(unused)]
fn main(){
    // tuple规则
    // 1.可以不同元素
    // 2.一旦申明长度不可变,也就是长度是已知的,索引以0开始
    // 3.声明解构
    // 4.点访问方式


    // 申明,可不同元素
    let tup :(i32,f64,u8,Name) = (500,6.5,3,Name{});

    //解构赋值,类型自动推断
    let (x,y,z,h) = tup.clone();

    //链式访问
    println!("name is :{:?}",tup.clone().3);

    //访问越界 panic
    // println!("name is :{:?}",tup.4)
}

#[derive(Debug,Clone)]
struct Name{}