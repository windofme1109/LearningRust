use std::os::windows::io::AsRawHandle;

fn main() {
    // 所有权与函数
    // 将值传递给函数，在语义上类似于对变量进行赋值
    // 所以，将变量传递给函数，将会触发移动或者是复制
    let s1 = String::from("hello");
    // s1 的值被移动到函数里面，所以，s1 被废弃
    take_ownership(s1);
    // 在将 s1 的值传入函数以后，s1 被废弃，任何对 s1 的使用都会报错
    //  borrow of moved value: `s1`
    // println!("s1 {}", s1);

    // x 是一个 i32 类型的，实现了 copy
    let x = 10;
    // 将 x 的值传入函数，实际上是将 x 的值复制了一份，传入函数中
    make_copy(x);
    // 函数执行完成后，x 依旧可以使用
    println!("x is {}", x);

    let s3 = give_ownership();
    println!("s3 {}", s3);
    let s4 = String::from("hello world4");
    // take_and_give_back 接收一个 String 类型，并将其返回
    // 传入的是 s4，因此 s4 的值会被移动到函数内，s4 被废弃
    // take_and_give_back 将传入的值返回，这个值会继续移动，移动到了 s5 上面
    // 所以，整体的所有权移动过程是：s4 -> s -> s5，s 是 take_and_give_back 的形参
    let s5 = take_and_give_back(s4);

    println!("s5 {}", s5);
    // error[E0382]: borrow of moved value: `s4`
    // println!("s4 {}", s4);

    // 变量所有权的转移总是遵循相同的模式：将一个值赋值给另一个变量时就会转移所有权
    // 当一个持有堆数据的变量离开作用域时，它的数据就会被 drop 清理回收
    // 除非这些数据的所有权移动到了另一个变量上

    let s6 = String::from("hello world 6");

    // s6 的值被移动到 s7 上面了，s6 被废弃
    let (s7, len) = calculate_length(s6);

    println!("s7 {}, len {}", s7, len);

}

fn take_ownership(some_string: String) { // some_string 进入作用域
    println!("some string {}", some_string);
}// some_string 离开作用域
// some_string 占用的内存会被释放

fn make_copy(some_integer: i32) {
    println!("some integer {}", some_integer);
}

// give_ownership 函数会将其返回值移动到调用它的函数的地方（值的移动）
fn give_ownership() -> String {
    let s = String::from("hello world");
    // s 作为函数的返回值，会移动到调用函数的地方
    return s;
}

// take_and_give_back 接收一个 String 类型，并将其返回
// 因此，s 的值会发生移动，即所有权会发生移动
fn take_and_give_back(s: String) -> String {
    return s;
}

// 调用函数时，保留对参数的所有权，可以通过元组形式，将参数作为元组的元素返回，元组的其他值时函数正常额返回值
// 这样可以将参数的值移动到调用 calculate_length 的函数内
fn calculate_length(some_string: String) -> (String, usize) {
    let l = some_string.len();
    return (some_string, l);
}