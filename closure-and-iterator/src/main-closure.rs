use std::thread;
use std::time::Duration;
fn main() {
    // println!("Hello, world!");

    let simulated_user_specified_value = 24;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );


    // Cacher 目前存在两个问题
    // 1. 同一个 Cacher 实例，调用了 value 函数，那么无论给 value 传什么值，最终都会得到第一次调用 value 函数时返回的值
    let mut c1 = Cacher::new(|x| x);
    let v1 = c1.value(1);
    let v2 = c1.value(2);

    // v1 1
    // v2 1
    println!("v1 {}", v1);
    println!("v2 {}", v2);

    // v2 和 v1 都是 1
    // 因为 Cache 只缓存第一次调用闭包的执行结果，后面无论再怎么调用，都不会执行闭包，生成新的值
    // 解决方式是：Cacher 存储一个哈希表而不是单一的值
    // 这个哈希表使用传入的 arg 值作为关键字
    // 并将关键字调用闭包后的结果作为对应的值
    // 相应地，value 方法不再简单地判断 self.value的值是 Some 还是 None
    // 而是会检查哈希映射里是否存在arg这个关键字
    // 如果存在的话，Cacher 就直接返回对应的值；如果不存在的话，则调用闭包，使用 arg 关键字将结果存入哈希表之后再返回
    // todo 将 value 字段替换为 hashmap

    // 2. Cacher 只能接收一个获取 u32 类型参数并返回u32类型的值的闭包
    // 但我们可能想要缓存的是一个获取字符串切片参数并返回 usize 值的闭包
    // 为了修复这一问题，我们可以自行尝试引入更多的泛型参数来提升 Cacher 功能的灵活性
    // todo 为闭包引入更改的泛型参数（trait 约束）


    // 使用闭包捕获环境上下文
    // 闭包可以捕获自己所在的环境并访问自己被定义时的作用域中的变量
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    // 正常编译通过，说明 equal_to_x(y) 返回的是 true
    assert!(equal_to_x(y));

    // 函数不具备这个功能，例如：

        let x = 4;
    // can't capture dynamic environment in a fn item
    // help: use the `|| { ... }` closure form instead
    // fn equal_to_x(z: i32) -> bool { z == x }
    //     let y = 4;
    //     assert!(equal_to_x(y));

    // 提示信息显示：不能在函数中捕获动态环境，编译器甚至直接建议我们使用闭包代替

    // 闭包可以通过 3 种方式从它们的环境中捕获值，这和函数接收参数的 3 种方式是完全一致的：获取所有权、可变借用及不可变借用
    // 这3种方式分别是 3 种 Fn 系列的 trait 中：
    // 1. FnOnce 意味着闭包可以从它的封闭作用域中，也就是闭包所处的环境中，消耗捕获的变量
    // 为了实现这一功能，闭包必须在定义时取得这些变量的所有权并将它们移动至闭包中
    // 这也是名称 FnOnce 中 Once 一词的含义：因为闭包不能多次获取并消耗掉同一变量的所有权，所以它只能被调用一次
    // 2. FnMut 可以从环境中可变地借用值并对它们进行修改
    // 3. Fn 可以从环境中不可变地借用值
    // 当你创建闭包时，Rust 会基于闭包从环境中使用值的方式来自动推导出它需要使用的 trait
    // 所有闭包都自动实现了 FnOnce，因为它们至少都可以被调用一次
    // 那些不需要移动被捕获变量的闭包还会实现 FnMut
    // 而那些不需要对被捕获变量进行可变访问的闭包则同时实现了 Fn
    //
    // 在上面的例子中，因为 equal_to_x 闭包只需要读取 x 中的值，所以它仅仅不可变地借用了 x 并实现了 Fn trait

    // 假如你希望强制闭包获取环境中值的所有权，那么你可以在参数列表前添加 move 关键字
    // 这个特性在把闭包传入新线程时相当有用，它可以将捕获的变量一并移动到新线程中去
    let x = vec![1, 2, 3];
    // error[E0382]: borrow of moved value: `x`
    // let equal_to_x_2 = move |y| y == x;
    // let y = vec![1, 2, 3];
    // 因为我们添加了 move 关键字，所以 x 的值会在定义闭包时移动至闭包中
    // 由于闭包拥有了 x 的所有权，所以 main 函数就无法在 println! 语句中使用 x 了
    // println!("can't use x here: {:?}", x);
    //
    // assert!(equal_to_x_2(y));

    // 在大部分情形下，当你需要指定某一个 Fn 系列的 trait时，可以先尝试使用 Fn trait
    // 编译器会根据闭包体中的具体情况来告诉你是否需要 FnMut 或 FnOnce
}

// 生成健康计划
// fn generate_workout(intensity: u32, random_number: u32) {

//     // 我们希望只调用一次 simulated_expensive_calculation 函数，因为使用这个函数的代价比较大，所以要尽量少的可能调用
//     // 可以将 simulated_expensive_calculation 函数的调用放在外面，使用一个变量接收
//     // 替换掉原来需要调用 simulated_expensive_calculation 的地方
//     let expensive_result = simulated_expensive_calculation(intensity);

//     if intensity < 25 {
//         // 在这个条件下，存在两次调用 simulated_expensive_calculation 这个函数的问题
//         // 结果是用户会等到比较长的时间
//         println!(
//             "Today, do {} pushups",
//             // simulated_expensive_calculation(intensity)
//             expensive_result
//         );

//         println!(
//             "Next, do {} situps",
//             // simulated_expensive_calculation(intensity)
//             expensive_result
//         );
//     } else {
//         // 
//         if random_number == 3 {
//             println!("Take a break  today! Remeber to stay hydrated!");
//         } else {
//             println!(
//             "Today, run for {} minutes",
//             // simulated_expensive_calculation(intensity)
//             expensive_result
//         );
//         }
//     }
// }


// 定义一个结构体，用来缓存闭包的计算结果
// 其接收一个泛型参数，该泛型参数实际上是一个闭包类型
//标准库中提供了一系列Fn trait，而所有的闭包都至少实现了 Fn、FnMut 及 FnOnce 中的一个 trait
// 根据我们对闭包的要求：闭包有一个u32类型的参数并返回一个u32值
// 因此我们需要对 trait 进行约束，添加参数类型和返回值类型，因此 trait 约束就是：Fn(u32) -> u32
//
struct Cacher<T> where T: Fn(u32) -> u32 {
    // Cacher 结构体拥有一个泛型 T 的calculation字段，而 trait 约束规定的这个T代表一个使用Fn trait 的闭包
    // 要求存储在 calculation 中的闭包必须有一个 u32 参数，同时必须返回一个u32值
    calculation: T,
    value: Option<u32>
}

// 注意：函数同样也可以实现这 3 个 Fn trait，假如代码不需要从环境中捕获任何值
// 那么我们也可以使用实现了 Fn trait 的函数而不是闭包

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    // Cacher::new 函数会接收一个泛型参数T，它与 Cacher 结构体有着相同的 trait 约束
    // 调用 Cacher::new 会返回一个在 calculation 字段中存储了闭包的 Cacher 实例
    // 因为我们还未执行过这个闭包，所以 value 字段的值被设置为了 None
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }
    // 当调用代码需要获得闭包的执行结果时，它们将会调用 value 方法而不是调用闭包本身
    // 这个方法会检查 self.value 中是否已经拥有了一个属于Some变体的返回值
    // 如果有的话，它会直接返回 Some 中的值作为结果而无须再次执行闭包
    // 而如果 self.value 是 None 的话 ， 则代码会执行 self.calculation 中的闭包
    // 将 返回值使用 Some 变体包裹并存储在 self.value 中以便将来使用，最后再把结果返回给调用者
    // 对于同一个实例，value 只调用一次闭包执行函数
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {

                let result = (self.calculation)(arg);
                self.value = Some(result);
                result
            }
        }
    }
}



// 优化版本
fn generate_workout(intensity: u32, random_number: u32) {

    // 使用 expensive_result 接收 simulated_expensive_calculation 的计算结果
    // 可以解决第一个 if 块中不必要的两次函数调用
    // 但是只要我们调用 generate_workout 这个函数，就会调用 simulated_expensive_calculation 函数进行计算
    // 这对于无须 expensive_result 结果值的内层 if 代码块来讲显得异常浪费
    // 我们希望在程序中将代码定义在一处，但只在真正需要结果时才执行相关代码
    // 可以通过定义闭包来解决这个问题

    // 闭包的定义
    // 等号左边是变量， 右边是闭包的函数体
    // 它会被赋值给语句左侧的 expensive_closure 变量。
    // 闭包的函数体以一对竖线（|）开始，并在竖线之间填写闭包的参数
    // 这个闭包仅有一个名为 num 的参数，而当闭包需要多个参数时，我们需要使用逗号来分隔它们，例如|param1, param2|
    // 在参数后面，我们使用了一对花括号来包裹闭包的函数体。如果这个闭包只是单行表达式，你也可以选择省略花括号
    // 在闭包结束后，也就是右花括号的后边，我们需要用一个分号来结束当前的let语句
    // 因为闭包代码中的最后一行（num）没有以分号结尾，所以该行产生的值会被当作闭包的结果返回给调用者，其行为与普通函数的完全一致
    // 注意：闭包并不强制要求你标注参数和返回值的类型
    // Rust 之所以要求我们在函数定义中进行类型标注，是因为类型信息是暴露给用户的显式接口的一部分
    // 严格定义接口有助于所有人对参数和返回值的类型取得明确共识
    // 但是，闭包并不会被用于这样的暴露接口：它们被存储在变量中，在使用时既不需要命名，也不会被暴露给代码库的用户
    // 闭包通常都相当短小，且只在狭窄的代码上下文中使用，而不会被应用在广泛的场景下
    // 在这种限定环境下，编译器能够可靠地推断出闭包参数的类型及返回值的类型，就像是编译器能够推断出大多数变量的类型一样
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // 使用存储在结构体中的闭包

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // 定义闭包
    // fn add_one_1(y) {x + 1}
    // let add_one_2 = |x| {x + 1};
    // let add_one_2 = |x| x + 1;
    // let add_one_3 = |x: i32| {x + 1};
    // let add_one_4 = |x: i32| x + 1;


    // 闭包定义中的每一个参数及返回值都会被推导为对应的具体类型
    // 定义一个闭包
    let example_closure = |x| x;
    // 第一次调用闭包，因此闭包的参数类型和返回值类型会被推导为第一次调用闭包时的参数和返回值类型，将 example_closure 闭包的参数 x 的类型和返回值的类型都推导为了 String 类型
    let s = example_closure(String::from("hello world"));

    // 第二次调用闭包，传入数字，就会报错
    // expected `String`, found `i32`
    // let n = example_closure(5);

    if intensity < 25 {
        // 在这个条件下，存在两次调用 simulated_expensive_calculation 这个函数的问题
        // 结果是用户会等到比较长的时间
        println!(
            "Today, do {} pushups",
            // simulated_expensive_calculation(intensity)
            // expensive_result
        //     调用闭包
        //     expensive_closure(intensity)
            // 将闭包存储在了新创建的 Cacher 实例中
            // 接着，在每一个需要结果的地方，我们都调用了 Cacher 实例的 value 方法。无论我们调用多少次 value 方法，或者一次都不调用，真正耗时的计算操作都最多只会执行一次
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps",
            // simulated_expensive_calculation(intensity)
            // expensive_result
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        // 
        if random_number == 3 {
            println!("Take a break  today! Remember to stay hydrated!");
        } else {
            println!(
            "Today, run for {} minutes",
            // simulated_expensive_calculation(intensity)
            // expensive_result
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        }
    }
}



// 模拟比较耗时的计算过程
fn simulated_expensive_calculation(intensity: u32) ->u32 {
    println!("calculating slowly...");
    // 延迟 2s
    thread::sleep(Duration::from_secs(2));
    intensity
}