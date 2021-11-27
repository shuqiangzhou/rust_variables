#![allow(unused)]

use std::string;
// 常量申明,申明常量用`const`关键字
// 调用另外一个常量是可行的
const MAX: u8 = std::u8::MAX;

// 常量不允许申明为`mut`
// const mut data:f64 = 3.16159;

// 常量名字申明必须为大写加下划线,且必须显示的申明类型
const PI: f64 = 3.14159;
#[derive(Debug)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

const NAME: Name = Name {
    first: "_",
    last: "_",
};

struct Bar {
    x: u8,
}
const BAR: Bar = Bar { x: 3 };
fn main() {
    let variable = 5;
    println!("x:{}", variable);

    // 此处会报错,申明处为不可变,所以这里不能再次赋值
    // variable = 6;
    let variable1 = variable;

    // 实现了copy语义的,他实际是一个深拷贝,所以上面那个实际上是一个拷贝操作,并不会吧原来的变量的所有权给交出来
    println!("variable:{}", variable);

    // 这段代码示例了未实现深拷贝
    let string = String::from("hello world");

    println!("string :{}", string);
    //当某一个字符串为实现深拷贝赋值给另外一个变量,源变量会不可用,因为已经把所有权交出来了
    let string1 = string;
    // 此处会报错因为string已经把所有权交给了string1,string已经不能再次使用了
    // println!("string :{:?}",string);

    // 可变变量需要在前面加一个`mut`

    let mut data = String::from("hello");

    println!("{}", data);
    data.push_str(" world");

    println!("{}", data);

    println!("const name:{:?}", NAME);
}
