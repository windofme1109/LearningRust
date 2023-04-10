// 第三章练习
fn main() {
    
    // println!("Hello, world!");

    // 
    let celsius_temperaute_1: f32 = 15.8;
    let fahrenheit_temperaute_1 = celsius_to_fahrenheit(celsius_temperaute_1);

    println!("摄氏度是：{}，对应的华氏温度是：{}", celsius_temperaute_1, fahrenheit_temperaute_1);

    let x1 = fib(3);
    let x2 = fib(5);
    let x3 = fib(10);
    let x4 = fib(20);

    println!("{}, {}, {}, {}", x1, x2, x3, x4);
}

// 摄氏温度与华氏温度的相互转换
// 摄氏度转华氏温度
fn celsius_to_fahrenheit(celsius_temperaute: f32) -> f32 {
    return celsius_temperaute * 1.8 + 32.0;
}

// 斐波那契数列数列 
// f(0) = 0，f(1) = 1，f(n) = f(n - 1) + f(n - 2) n >= 2
fn fib(n: i32) -> i64 {
    let mut n1 = 0;
    let mut n2 = 1;

    if n <= 1 {
        return n2;
    } 


    for i in 2..n+1 {
        let temp = n1 + n2;
        n1 = n2;
        n2 = temp;
    }

    return n2;
}
