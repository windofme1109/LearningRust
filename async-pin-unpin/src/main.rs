/// 异步编程中的 Pin 与 Unpin

// 在 Rust 中，所有的类型可以分为两类:
// 1. 类型的值可以在内存中安全地被移动，例如数值、字符串、布尔值、结构体、枚举，总之你能想到的几乎所有类型都可以落入到此范畴内
// 2. 自引用类型，数据及其引用存放在一处。也就是说如果一个结构，引用或者指针指向了自己的数据，那么它就是一个自引用结构
//    关于自引用，可以参考：https://zhuanlan.zhihu.com/p/600784379
//    https://zhuanlan.zhihu.com/p/601908172                 
// 例如：
// 这里使用裸指针定义了一个自引用结构
struct Test {
    a: String,
    b: *const String,
}

// a 是一个字符串
// 而 b 则是这个字符串的原始指针
// a 的数据来自于 new 关联函数的参数 txt，b 在 new 的时候是一个空指针，在 init 的时候变成 a 字段的指针
// a 和 b 俩方法返回的数据理论上是一样的(b的错误场景不考虑)

// 注意：这里只能是原始指针，如果是引用指针就需要声明生命周期才行，但是这种自引用类型是没有办法的。



impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        // 获取 a 的原始指针
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}


fn main() {
    // println!("Hello, world!");
    
    
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");
    test1.init();
    test2.init();

    // a: test1, b: test1
    println!("a: {}, b: {}", test1.a(), test1.b());
    // a: test2, b: test2
    println!("a: {}, b: {}", test2.a(), test2.b());

    // 自引用的问题
}

// Pin 
// Pin 的本意是别针，那么其作用就很明显了，将某个类型别住，防止其移动
// 实际上，Pin 类型的主要作用是：防止一个类型在内存中被移动
// 上面的自引用类型存在的问题是，如果指针指向的内存地址变了，而指针依旧指向原来的内存地址，那么就会发生错误
