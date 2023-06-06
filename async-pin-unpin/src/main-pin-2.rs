use std::marker::PhantomPinned;
use std::pin::Pin;

// 这里使用裸指针定义了一个自引用结构
struct Test {
    a: String,
    b: *const String,
}

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


// Pin
// Pin 的本意是别针，那么其作用就很明显了，将某个类型别住，防止其移动
// 实际上，Pin 类型的主要作用是：防止一个类型在内存中被移动
// 上面的自引用类型存在的问题是，如果指针指向的内存地址变了，而指针依旧指向原来的内存地址，那么就会发生错误

// 理解了 Pin 的作用，我们可以使用 Pin 来修改上面的自引用的例子

// 1. 将值固定到栈上
#[derive(Debug)]
struct Test2 {
    a: String,
    b: *const String,
    // 标记类型 PhantomPinned 将自定义结构体 Test2 变成了 !Unpin (编译器会自动帮我们实现)
    // 因此该结构体无法再被移动
    // PhantomPinned 是一个不会实现 Unpin trait 的标记类型，如果一个类型包含 PhantomPinned，那它默认不会实现 Unpin
    _marker: PhantomPinned
}

impl Test2 {
    fn new(txt: &str) -> Self {
        Test2 {
            a: String::from(txt),
            b: std::ptr::null(),
            // 这个标记可以让我们的类型自动实现特征 !Unpin
            //
            _marker: PhantomPinned
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        // Test2 被标记为 Pin，那么我们需要在不安全的代码块中，调用不安全的函数 get_unchecked_mut
        // 获得 Pin 包裹的 Test2 中的数据的可变引用
        let this = unsafe {self.get_unchecked_mut()};
        // 将 a 的指针赋予 b，这里不是赋予 self.b，而是 this.b
        // this 就是刚刚说的，Pin 包裹的 Test2 的可变引用
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        // get_ref 函数可以用来获取 Pin 中包裹的类型的共享指针
        // 这是安全的因为我们不可能移动一个共享指针
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}

// 注意：
// 一旦类型实现了 !Unpin，那将它的值固定到栈( stack )上就是不安全的行为
// 因此在代码中我们使用了 unsafe 语句块来进行处理，你也可以使用 pin_utils 来避免 unsafe 的使用


// 2. 固定到堆上

// 将一个 !Unpin 类型的值固定到堆上，会给予该值一个稳定的内存地址，它指向的堆中的值在 Pin 后是无法被移动的
// 而且与固定在栈上不同，我们知道堆上的值在整个生命周期内都会被稳稳地固定住

#[derive(Debug)]
struct Test3 {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
    
}

impl Test3 {

    // 实例化的时候，使用 Box::pin 进行包裹
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test3 {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };

        // 关键是 Box::Pin
        // 构造一个新的 Pin<Box<T>>
        // 如果 T 没有实现 Unpin，那么 x 将被固定在内存中，无法移动
        let mut boxed = Box::pin(t);

        // 获得 a 的指针
        //
        let self_ptr: *const String = &boxed.as_ref().a;

        // 将获得到的 a 指针赋给 b
        // as_mut 方法可以从 pin 住的指针获取 pin 住的可变引用
        // get_unchecked_mut 方法可以获取此 Pin 住的数据的可变引用
        // 拿到被 pin 住的 t 的引用
        // get_unchecked_mut 方法是不安全的方法，因此需要放在 unsafe 代码块中
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}
fn main() {

    //  2. 固定到堆上

    let test1 = Test3::new("test1");
    let test2 = Test3::new("test2");

    // as_ref
    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());


    // let mut test1 = Test::new("test1");
    // let mut test2 = Test::new("test2");
    // test1.init();
    // test2.init();
    //
    // // a: test1, b: test1
    // println!("a: {}, b: {}", test1.a(), test1.b());
    // // a: test2, b: test2
    // // println!("a: {}, b: {}", test2.a(), test2.b());
    //
    // // 自引用的问题
    //
    // // 交换 test1 和 test2 的值
    // // swap 方法用来交换两个可变位置的值
    // // 那么 test1 和 test2 指向的位置不变，但是值被交换了
    // std::mem::swap(&mut test1, &mut  test2);
    // // 交换以后，test2 指向的位置的值是：test1，理论上应该打印出：a: test1, b: test1
    // // 但实际上，打印出的是：a: test1, b: test2
    // println!("a: {}, b: {}", test2.a(), test2.b());
    // // 出现上面的情况的原因是：
    // // test2.b 指针依然指向了旧的地址，而该地址对应的值现在在 test1 里，最终会打印出意料之外的值
    // // 我们可以做一个验证，修改 test1 中的 a 的值：
    // test1.a = "I've totally changed now!".to_string();
    //
    // // a: test1, b: I've totally changed now!
    // println!("a: {}, b: {}", test2.a(), test2.b());

    // test1 是一个结构体，里面由两个属性 a 和 b，因为 a 是字符串类型
    // 所以，a 存储的是一个内存地址，指向具体的字符串存放地址，而 b 是一个指向 a 的指针，所以，b 存储的就是 a 的内存地址
    // 假设 a 在栈内存的地址为 0x1002，其存储的值为 0x1111
    // 而 b 在栈内存的地址为 0x1003，其存储的值为 0x1002

    // 同理，test2 中 a 和 b 的位置以及存储的值的情况为：
    // a 在栈内存的地址为 0x2002，其存储的值为 0x2222
    // 而 b 在栈内存的地址为 0x2003，其存储的值为 0x2002

    // 交换 test1 和 test2，那么是对应属性的进行交换，也就是 a 和 a 交换，b 和 b 交换
    // 此时内存分布如下：
    // test1
    // a 在栈内存的地址为 0x1002，其存储的值为 0x2222
    // b 在栈内存的地址为 0x1003，其存储的值为 0x2002

    // test2
    // a 在栈内存的地址为 0x2002，其存储的值为 0x1111
    // b 在栈内存的地址为 0x2003，其存储的值为 0x1002

    // 由上面的指针分布情况可以看出，
    // test1.b 指向了 test2.a
    // test2.b 指向了 test1.a
    // 所以此时的 b 属性并没有指向属于自己结构体的 a 属性
    //
    // 所以，修改 test.a 的值，test2.b 也会跟着变化

    // 使用 Pin 固定值
    // 此时的`test1`可以被安全的移动
    // let mut test3 = Test2::new("test3");
    //
    // // 新的 test3`由于使用了 Pin，因此无法再被移动，这里的声明会将之前的 test1 遮蔽掉(shadow)
    // // 使用 Pin 固定住 test3
    // // new_unchecked 是结构体 Pin 的一个关联方法
    // // 围绕对可能实现也可能不实现 Unpin 的某些数据类型的引用构造一个新的 Pin<P>
    // // 如果指针取消引用到 Unpin 类型，则应改用 Pin::new
    // let mut test3 = unsafe { Pin::new_unchecked(&mut test3) };
    //
    // // as_mut 可以从一个被 pin 住的指针中获取 被 pin 住的可变引用，这是一个泛型方法
    // Test2::init(test3.as_mut());
    //
    // let mut test4 = Test2::new("test4");
    // let mut test4 = unsafe { Pin::new_unchecked(&mut test4) };
    // Test2::init(test4.as_mut());
    //
    // println!("a: {}, b: {}", Test2::a(test3.as_ref()), Test2::b(test3.as_ref()));

    // 此时，我们无法交换 test3 和 test4 的值
    // 报错信息如下：
    // error[E0277]: `PhantomPinned` cannot be unpinned
    //    --> src\main-pin-2:158:26
    //     |
    // 158 |     std::mem::swap(test3.get_mut(), test4.get_mut());
    //     |                          ^^^^^^^ within `Test2`, the trait `Unpin` is not implemented for `PhantomPinned`
    //     |
    //     = note: consider using `Box::pin`
    // std::mem::swap(test3.get_mut(), test4.get_mut());
    // println!("a: {}, b: {}", Test2::a(test4.as_ref()), Test2::b(test4.as_ref()));


    // todo 这块不是很理解
    //
    // 需要注意的是：固定在栈上非常依赖于我们写出的 unsafe 代码的正确性
    // 我们知道 &'a mut T 可以固定的生命周期是 'a ，但是我们却不知道当生命周期 'a 结束后该指针指向的数据是否会被移走
    // 如果你的 unsafe 代码里这么实现了，那么就会违背 Pin 应该具有的作用

    // 一个常见的错误就是忘记去遮蔽( shadow )初始的变量
    // 因为你可以 drop 掉 Pin ，然后在 &'a mut T 结束后去移动数据:

    // let mut test1 = Test2::new("test1");
    // let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    // Test2::init(test1_pin.as_mut());
    //
    // // 释放 test1_pin
    // drop(test1_pin);
    // // 此时 test1 并不是被 Pin 住
    // // test1.b points to "test1": 0x6e9a2ff740...
    // println!(r#"test1.b points to "test1": {:?}..."#, test1.b);
    //
    // let mut test2 = Test2::new("test2");
    //
    // // 交换 test1 和 test2
    // // 因为 test1 和 test2 都没有被 pin 住，因此可以进行交换
    // std::mem::swap(&mut test1, &mut test2);
    // // ... and now it points nowhere: 0x0
    // println!("... and now it points nowhere: {:?}", test1.b);

}