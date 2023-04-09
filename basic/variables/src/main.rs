// 声明一个常量
// 声明常量必须使用 const 关键字
// 声明一个常量必须指定数据类型
// 常量可以被声明在任何作用域中，甚至包括全局作用域。
//
// 只能将常量绑定到一个常量表达式上，而无法将一个函数的返回值，或其他需要在运行时计算的值绑定到常量上
// 常量的变量名通常是下画线分隔的全大写字母，如果是数值，则可以在数值中插入下画线来提高可读性
const MAX_POINTS:u32 = 100_000;

fn main() {

    // 不需要指定变量的数据类型，Rust 会自动推导
    // 默认情况下，rust 声明的变量是不能改变的，也就是一个常量，如果中途修改，在编译过程中会报错
    // 如果向让变量可以变化，使用 mut 关键字进行声明
    let mut x = 5;

    println!("The value of x is {}", x);

    // cannot assign twice to immutable variable
    x = 6;

    println!("The value of x is {}", x);


    // shadow
    // 在 Rust 中，可以使用 let 声明多个同名的变量，前面的同名变量会被隐藏（shadow），也可以理解为是后面的变量遮挡了前面的变量
    // 使用时，永远于其前面的变量为准

    let b = 100;
    // b = 100 + 1 = 101
    let b = b + 1;
    // b= 101 * 2 = 202
    let b = b * 2;
    // the value of b is 202
    println!("the value of b is {}", b);
    // shadow 的优点是，可以声明多个同名变量
    // 同时这个变量的数据类型还可以变化
    // 而使用 mut 声明的变量，数据类型是不可变的
    let a = "hello";

    let a = a.len();
    // the value of a is 5
    println!("the value of a is {}", a);

    // let mut b = "    ";
    // // expected `&str`, found `usize`
    // b = b.len();



    // 数据类型 - 标量类型与复合类型
    // 标量类型
    // 整数的默认类型是 i32
    let z = 100;
    // 指定整数类型
    let zz: i64 = 200;


    // 浮点数的默认类型是 f64
    let x = 2.5;
    let y: f32 = 55.0;

    // 加减乘除、取余
    // 会根据数据的类型自动推断结果的类型
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 54 % 5;

    // 布尔值
    // true 和 false
    // 标识是 bool
    // 占用一个字节
    let t = true;
    let f: bool = false;

    // 字符类型
    // char类型被用于描述
    // 语言中最基础的单个字符
    // char类型占 4 字节，是一个Unicode标量值，这也意味着它可以表示比 ASCII 多得多的字符内容。
    // 可以表示拼音字母、中文、日文、韩文、零长度空白字符，甚至是emoji表情
    // 实际上，Unicode标量可以描述从 U+0000 到 U+D7FF 以及从 U+E000 到 U+10FFFF 范围内的所有值

    let z = 'x';
    let z: char = 'β';
    let z = '😝';

    // 复合类型
    // Rust 中，复合类型包括两个：元组和数组
    // 元组（tuple）可以将其他不同类型的多个值组合进一个复合类型中。
    // 元组还拥有一个固定的长度：你无法在声明结束后增加或减少其中的元素数量
    // 为了创建元组，我们需要把一系列的值使用逗号分隔后放置到一对圆括号中
    // 元组每个位置的值都有一个类型，这些类型不需要是相同的

    // 定义一个元组，并显式声明其类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 获取元组中的值，可以通过解构的方式
    let (k, m, n) = tup;
    println!("k is {}, m is {}, n is {}", k, m, n);

    // 访问 tuple 中变量的方式
    // 使用点标记法，后接元素的索引编号
    println!("first is {}, second is {}, third is {}", tup.0, tup.1, tup.2);

    // 数组
    // 数组与元组类似，也是固定长度的用来存放多个值的复合类型
    // 但是数组只能存放同一种数据类型
    // 数组一旦定义，其长度不可改变，当然，Rust 标准库也提供了一个更加灵活的动态数组（vector）类型
    // 动态数组是一个类似于数组的集合结构，但它允许用户自由地调整数组长度
    let arr = [1, 2, 3, 4, 5];

    let month = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    // 也可以这样定义数组：指定数组元素的类型和长度
    let arr2: [i32; 3] = [1, 2, 3];
    // [i32; 3] 定义了数组的类型和长度，i32 是数组元素的类型，3 是数组长度，中间用分号隔开

    // 如果数组的元素都相同，可以通过一种快捷的方式建立数组
    let arr3 = [3; 5];
    // [3; 5] 3 是数组的元素，5 是数组的长度，中间用分号隔开，等价于：let arr3 = [3, 3, 3, 3, 3];

    // 通过索引的方式访问数组: 方括号加上索引 - [index]
    let first = month[0];
    let second = month[1];
    println!("first month is {}, second month is {}", first, second);


    // 索引超出了最后一个元素的索引，及访问数组中不存在的索引
    // 在 build 过程中就会出错，提示 index out of bounds: the length is 3 but the index is 10
    // let index = 10;
    // //
    // let ele = arr2[index];
    // println!("The value of element is: {}", ele);

    // 下面这种形式，编译可以通过，但是运行时会报错
    // 目前编译只会检查第一层的索引
    // let index2 = [10, 11, 12];
    // index out of bounds
    // let ele2 = arr2[index2[1]];
    // println!("The value of element is: {}", ele2);\

    // 调用函数
    another_function();

    add(10, 20);

    let res = five();
    println!("the value of res is {}", res);
    let res2 = add_five(10);
    println!("the value of res2 is {}", res2);

    // 条件控制 if else
    // if 结构是一个条件表达式，
    // 因此 if 后面必须跟一个判断条件，这个判断表达式必须返回一个布尔值
    // if表达式中与条件相关联的代码块也被称作分支 (arm)
    // 表达式不需要加括号，且不会进行类型转换

    let number = 100;
    if number < 150 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // if {} else if {} else {}
    // 可以用来处理多重分支的情况
    // 过多的else if表达式可能会使我们的代码变得杂乱无章，可以使用另外一个强大的分支结构语法 match 来取代 if elseif 这种形式
    let num = 101;
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }


    // 由于if是一个表达式，所以我们可以在let语句的右侧使用它来生成一个值
    // num2 的值会被绑定到 if 表达式执行结果
    // 注意：代码块输出的值就是其中最后一个表达式的值
    // 另外，数字本身也可以作为一个表达式使用
    // 整个if表达式的值取决于究竟哪一个代码块得到了执行
    let num2 = if 100 > 10 {100} else {10};
    println!("num2 is {}", num2);

    // if {} else {} 用作生成一个值，要求所有 if 分支返回的类型都相同
    // 如果不相同，编译过程就会报错
    // 因为变量只能拥有单一的类型，所以下面这段代码无法通过编译
    // 为了对其他使用 num3 变量的代码进行编译时类型检查，Rust需要在编译时确定number的具体类型
    // `if` and `else` have incompatible types expected integer, found `&str`
    // let num3 = if 2 < 10 { 2 } else {"10"};


    // 循环
    // Rust 提供了三种方式的循环：loop、while 和 for

    // loop
    // loop {
    //     println!("loop");
    // }
    // loop 需要显示的调用 break，才能终止循环
    // loop 本身也是一个表达式，因此可以返回一个值
    // loop 也可以返回值
    // 在loop内部 break 的后面可以跟一个表达式，这个表达式就是最终的 loop表达式的值。
    let mut count = 0;
    let result = loop {
        count = count + 1;
        if count == 10 {
            break count * 20; // 加不加分号均可
        }
    };

    println!("final result is {}", result);


    // while 循环，每一次循环都需要判断条件，条件成立，循环继续，条件不成立，循环终止
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;

    }

    println!("LIFTOFF!!!");

    // 使用 for 循环遍历数组
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // 不过不推荐使用 while 循环遍历数组，因为需要手动维护索引和数组的长度，非常容易出错

    // for 循环（for in）
    // 比 while 循环简洁
    // 直接取出 a 中的元素，因此不需要维护索引和数组的长度
    // iter 函数返回数组的迭代器，可以用 for in 遍历
    //
    for element in a.iter() {
        println!("for the value is: {}",element);
    }


    // 标准库提供了一个 Range 类型，可以生成指定区间的数字
    // 接收一个开始数字和一个结束数字，Range 生成他们之间的数字（不包括结束数字）
    // 形式是：(start..end)，例如：(1..4) 生成 1、2、3 这三个数字，当然，Range 返回的是要给迭代器，可以使用 for in 循环
    // rev 函数可以反转 range 生成的数字的顺序
    // 使用 for 实现 while 循环的倒计时案例
    for count in (1..4).rev() {
        println!("{}!", count);
    }
    println!("LIFTOFF!!!");

    let y = {
        let x= 100;
        x + 1
    };
    // 101
    println!("new y is {}", y);
}

// 函数

// 使用 fn 关键字定义函数
// 函数命名使用蛇形命名法
fn another_function() {
    println!("this is another function");
}

// 函数可以接收参数，参数类型必须在定义函数时指定
fn add(x: i32, y: i32) {
    println!("x is {}", x);
    println!("y is {}", y);
}

// 表达式（expression）与语句（statement）


// 函数的返回值
// 在函数定义中，声明函数的返回值，形式是：fn fn_name() -> i32 {}
// Rust 中的函数返回值，默认是函数体中最后一个表达式的值
fn five() -> i32 {
    // 下面是一个表达式，没有分号（加了分号就成了语句），而且是函数体中最后一个表达式，所以5 就是函数的返回值
    5
}

// 如果想让函数提前返回，可以使用 return，return 可以放在函数的任何地方，返回相应的类型
// 推荐使用 return，予以更明确

fn add_five(x: i32) -> i32 {
    return x + 5;
}

// 如果函数中，没有 return，最后一行也不是表达式，那么函数默认返回一个空的元组

// 语句与表达式
// 在 Rust 中，
// 语句指那些执行操作但不返回值的指令
// 表达式 则是指会进行计算并产生一个值作为结果的指令
// 进一步说，在维基百科的定义中，表达式是指由变量、常量和操作符等组合成的可求值的语法实体。
// 比如说由数字、变量组成的加减乘除操作，表达式一定会产生一个结果，因此，单独的一个数字、字符等也是一个表达式
// Rust 中表达式有很多种，如赋值表达式、字面量表达式、块表达式等
// 
// 而语句一般分为两种：声明语句、表达式语句
// 1. 声明语句，用于声明变量、结构体、函数等各种项以及通过use等关键字引入包等。
// 2. 表达式语句，由一个表达式和一个分号组成，即在表达式后面加一个分号就将一个表达式转变为了一个语句
// 语句不返回值，因此其类型是 ()，一个空的tuple
// 从这个角度来看，可以将Rust看作一切皆表达式。由于当分号后面什么都没有时自动补单元值（）的特点，
// 可以将Rust中的语句看作计算结果均为（）的特殊表达式。而对于普通的表达式来说，则会得到正常的求值结果。
// 由于语句不返回值，所以不能将语句赋值给一个变量
// expected expression, found `let` statement
// let ww = (let zz = 100);


// 语句与表达式的区别是带不带分号，一个带了分号的表达式，就是一个语句

// 用花括号（{}）包裹的内容，也会被是作为一个表达式，因此，总是将花括号内最后一个表达式的结果，当做整个表达式的结果
// 100
// let x = {
//     println!("hello");
//     100
// }
// ()
// let x = {
//     println!("hello");
//     100;
// }

// 函数体由若干条语句组成，并可以以一个表达式作为结尾
// 函数体本身就可以看成是一个表达式，因此，可以将函数体内部结尾的表达式的值作为函数的返回值





