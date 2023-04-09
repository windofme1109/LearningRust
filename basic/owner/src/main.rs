fn main() {
    // 切片
    // 切片是另外一种不持有数据所有权的数据结构
    // 切片包括字符串切片、数字切片等多种形式
    // 切片是一种不可变引用

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // word 5
    println!("word {}", word);
    // word 是一个索引，可以用来从 s 中取出第一个单词
    // 但是这里存在一个问题，就是 word 应该和 s 是绑定的
    // 获得 s 的第一个单词，是 word 和 s 一起才能获取，单独一个 word 没有任何意义
    // 但是如果 s 发生了变化比如 s 变短了，或者是 s 被清除了 s.clear();，那么 word 就没有任何意义了

    // 因此，我们需要一种和字符串结合的更紧密的数据，这就是切片

    // 字符串切片
    // 字符串切片是指向String对象中某个连续部分的引用
    // 与获得字符串的引用类型，但是加上了起止索引
    // 在一对方括号中指定切片的范围区间[starting_index..ending_index]
    // 其中的 starting_index 是切片起始位置的索引值，ending_index 是切片终止位置的下一个索引值
    // ending_index - starting_index 是切片字符串的长度
    // 切片数据结构在内部存储了指向起始位置的引用和一个描述切片长度的字段
    let slice1 = &s[0..5];
    let slice2 = &s[6..11];

    println!("slice 1 {}, slice 2 {}", slice1, slice2);

    // 切片语法糖
    // 1. 省略起始为 0 的索引
    // slice1 与 slice2 等价
    let slice1 = &s[0..5];
    let slice2 = &s[..5];
    println!("slice 1 {}, slice 2 {}", slice1, slice2);
    // 2. 省略结束位置为字符串长度的索引
    // slice1 与 slice2 等价
    let len = s.len();
    let slice1 = &s[6..len];
    let slice2 = &s[6..];
    println!("slice 1 {}, slice 2 {}", slice1, slice2);
    // 3. 获得一个完整的字符串的切片
    // slice1 与 slice2 等价
    let len = s.len();
    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("slice 1 {}, slice 2 {}", slice1, slice2);
    // 注意：字符串切片的边界必须位于有效的UTF-8字符边界内

    // 调用使用切片的 first_word_slice 函数
    let hello = first_word_slice(&s);

    // hello 是一个切片
    // 实际上是对 s 的一个不可变引用，因此后面不能再有对 s 的可变引用
    // 由于 clear 函数需要截断当前 String 实例，所以调用 clear 需要传入一个可变引用。这就是编译失败的原因
    // 使用了切片，可以帮助我们在编译阶段发现这种错误
    //  cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();

    // first is hello
    println!("first is {}", hello);

    // 字符串字面量就是切片
    // 所以字符串字面量的类型就是 &str
    // 变量s1的类型其实就是 &str：它是一个指向二进制程序特定位置的切片
    // 正是由于 &str 是一个不可变的引用，所以字符串字面量自然才是不可变的
    let s1 = "hello world";

    let hello_1 = first_word_slice_params(s1);
    let hello_2 = first_word_slice_params(&s[..]);
    println!("hello_1 is {}, hello_2 is {}", hello_1, hello_2);

    // 其他类型的切片
    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    // 数组的切片类型是&[i32]，它在内部存储了一个指向起始元素的引用及长度，这与字符串切片的工作机制完全一样。
    let arr_s1 = &arr[1..3];
}

// 为什么需要切片

// 定义一个函数，取出字符串中第一个空格出现的位置，如果找不到第一个单词，就返回整个字符串的长度
fn first_word(s: &String) -> usize {

    // 1. 使用 as_bytes 方法将 String 转换为字节数组，因为我们的算法需要依次检查 String 中的字节是否为空格。
    // 2. 通过 iter 方法创建了一个可以遍历字节数组的迭代器
    // 3. enumerate 则将 iter 的每个输出作为元素逐一封装在对应的元组中返回
    //    元组的第一个元素是索引，第二个元素是指向集合中字节的引用。使用 enumerate 可以较为方便地获得迭代索引

    // (i, &item) 表示使用 模式匹配的形式对 enumerate 方法返回的元组的访问
    for (i, &item) in s.as_bytes().iter().enumerate() {
        // 以字节字面量的形式声明空的的字节形式，因为 as_bytes 方法获得的就是字符串对应的字节数组（ascii）
        if item == b' ' {
            return i;
        }
    }

    return s.len();

}

// 使用切片定义新的 first_word
fn first_word_slice(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        // 以字节字面量的形式声明空的的字节形式，因为 as_bytes 方法获得的就是字符串对应的字节数组（ascii）
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

// 使用切片作为参数，提高函数的通用性
// 因为字符串字面量本身就是切片，而 String 类型也可以转换为切片
// 所以使用切片类型，使得函数更具通用性
fn first_word_slice_params(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        // 以字节字面量的形式声明空的的字节形式，因为 as_bytes 方法获得的就是字符串对应的字节数组（ascii）
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}