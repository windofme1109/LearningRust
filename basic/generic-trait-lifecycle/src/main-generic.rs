
// Rust 中的泛型

fn largest_num(arr: &[i32]) -> i32 {
    let mut largest = arr[0];

    // arr 是一个 i32 类型的切片，迭代器中每个元素是 &i32
    // &item 这样引用形式，item 就是 i32，这种形式是解构
    for &item in arr {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(arr: &[char]) -> char {
    let mut largest = arr[0];

    
    for &item in arr {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// 使用泛型进行优化

// 泛型，就是类型参数，我们不在定义函数时确定参数或者返回值类型，而是使用泛型表示参数或者返回值类型
// 当真正使用函数时，将泛型确定为具体的参数类型

// 任何合法标识符的都可以作为类型参数名称。但出于惯例，我们选择了T
// 在Rust中，我们倾向于使用简短的泛型参数名称，通常仅仅是一个字母
// Rust采用了驼峰命名法（CamelCase）作为类型的命名规范，T 作为“type”的缩写，往往是大部分Rust程序员在命名类型参数时的默认选择

// 当我们需要在函数签名中使用类型参数时，也需要在使用前声明这个类型参数 的名称
// 为了定义泛型版本的 largest 函数，类型名称的声明必须被放置在函数名与参数列表之间的一对尖括号<>中，如下所示

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

    
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// 上面的 largest 可以理解为：
// 函数largest拥有泛型参数T，它接收一个名为list的T值切片作为参数，并返回一个同样拥有类型T的值作为结果


// 定义结构体使用泛型
struct Point<T> {
    x: T,
    y: T
}

// 在结构体中使用多个泛型参数
struct Point_2<T, U> {
    x: T,
    y: U
}

//  在枚举中使用泛型类型

enum IpAddr<T, U> {
    IPV4(T),
    IPV6(U)
}
// enum Option<T> {
//     Some(T),
//     None,
// }
    
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// 内置枚举 Result、Option 都使用了泛型参数
// 这样使用的时候非常灵活，可以根据我们的要求，使用不同类型的变体

// 方法中也可以使用泛型

struct Point_3<T> {
    x: T,
    y: T,
}

// 注意，我们必须紧跟着 impl 关键字声明 T，以便能够在实现方法时指定类型 Point_3<T>
// 通过在 impl 之后将 T 声明为泛型，Rust能够识别出 Point 尖括号内的类型是泛型而不是具体类型
// 可以这样理解：impl 后面定义的泛型 T 是一个形参，它会作为 Point_3 的泛型传给 Point_3，Point_3<T> 里的 T，从 Point_3 就是实参
// 这样定义的方法，对于所有的 Point_3<T> 的实例，都有效，即 Point_3<T> 的实例都可以调用 x 方法
impl<T> Point_3<T>  {
    fn x(&self) -> &T {
        &self.x
    }
    
}

// 定义方法时指定具体的类型，那么该方法只能是这个具体类型的 Point_3 实例才能使用
// 类 型 Point<f64> 将 会 拥 有 一 个 名 为 distance_from_origin 的方法，
// 而其他的Point<T>实例则没有该方法的定义
impl Point_3<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 方法的泛型也可以不与结构体的泛型相同。可以拥有自己的泛型参数
struct Point_4<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point_4<T, U> {
    // 这个方法会接收另外一个 Point_4 作为参数，而它与 self 参数所代表的 Point_4 之间有可能拥有不同的类型
    // 方法在运行结束后会创建一个新的 Point_4 实例
    // 这个实例的 x 值来自 self 所绑定的 Point_4（拥有类型T），而y值则来自传入的Point_4（拥有类型W）
    fn mixup<V, W>(self, other: Point_4<V, W>) -> Point_4<T, W> {
        Point_4 { x: self.x, y: other.y }
    }
}

// 注意，Rust实现泛型的方式决定了使用泛型的代码与使用具体类型的代码相比不会有任何速度上的差异

fn main() {
    println!("Hello, world!");

    let list = [1, 2, 10, 5, 3, 4];
    let char_list = ['a', 'b', 'e', 'c', 'h'];

    let largest_1 = largest_num(&list);
    let largest_char = largest_char(&char_list);

    println!("largest ele is {}", largest_1);
    println!("largest char is {}", largest_char);

    // 使用带泛型的 largest 函数
    // 对于 largest 函数这种内部需要比较的场景，直接使用泛型，目前还不能通过编译
    // 因为 T 是运行时才知道是什么类型，那么能不能直接用于比较是未知的
    // 根据在编译过程中的提示，T 需要实现 std::cmp::PartialOrd 这个 trait 才可以
    // let largest_2 = largest(&list);

    // Point 使用了泛型定义，而 x 和 y 是属于同一个类型 T，因此，在定义实例是，x 和 y 必须是相同类型的，如果类型不同，就会报错
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    // 当我们将整数5赋值给x时，编译器就会将这个 Point<T> 实例中的泛型T识别为整数类型
    // 但是，我们接着为y指定了浮点数4.0，而这个变量被定义为与 x 拥有相同类型，
    // 因此这段代码就会触发一个类型不匹配错误
    // error[E0308]: mismatched types
    // let other = Point {x: 5, y: 4.0};

    // 多个泛型类型
    let  p1 = Point_2 {x: 10, y: 5.0};

    let p2 = Point_3 {x: 100, y: 50};

    // distance is 5
    let p3 = Point_3 {x: 3.0, y: 4.0};

    // p2.x = 100
    println!("p2.x = {}", p2.x());
    println!("distance is {}", p3.distance_from_origin());

    let p5 = Point_4 {x: 10, y: 10};
    let p6 = Point_4 {x: 5.0, y: "hello"};

    let p7 = p5.mixup(p6);
    // p7.x = 10, p7.y = hello
    println!("p7.x = {}, p7.y = {}", p7.x, p7.y)
}
