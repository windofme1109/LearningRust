
// 并发中的消息通信

// Rust在标准库中实现了一个名为通道（channel）的编程概念，它可以被用来实现基于消息传递的并发机制
// 你可以将通道想象为有活水流动的通道，比如小溪或河流。只要你将橡皮鸭或小船这样的东西放入其中，它就会顺流而下抵达水路的终点

// 编程中的通道由发送者（transmitter）和接收者（receiver）两个部分组成
// 发送者位于通道的上游，也就是你放置橡皮鸭的地方；而接收者则位于通道的下游，也就是橡皮鸭到达的地方
// 某一处代码可以通过调用发送者的方法来传送数据
// 而另一处代码则可以通过检查接收者来获取数据
// 当你丢弃了发送者或接收者的任何一端时，我 们就称相应的通道被关闭

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    // 使用 mpsc::channel 函数创建了一个新的通道
    // 路径中的 mpsc 是英文 “multiple producer, single consumer”（多个生产者，单个消费者）的缩写
    // 简单来讲，Rust标准库中特定的实现方式使得通道可以拥有多个生产内容的发送端 ，但只能拥有一个消耗内容的接收端
    // 函数 mpsc::channel 会返回一个含有发送端与接收端的元组。代码中用来绑定它们的变量名称为 tx和rx
    // 这也是在许多场景下发送者与接收者的惯用简写。这里还使用了带有模式的let语句对元组进行解构
    // let (tx, rx) = mpsc::channel();


    // 使用 thread::spawn 生成了一个新线程
    // thread::spawn(move || {
    //
    //     let val = String::from("hi");
    //
    //     // 为了让新线程拥有 tx 的所有权，我们使用 move 关键字将 tx 移动到了闭包的环境中
    //     // 新线程必须拥有通道发送端的所有权才能通过通道来发送消息
    //     // 现在 tx 所有权在闭包内
    //     // 发送端提供了 send 方法来接收我们想要发送的值
    //     // 这个方法会返回 Result<T, E> 类型的值作为结果；当接收端已经被丢弃而无法继续传递内容时
    //     // 执行发送操作便会返回一个错误，这里我们在出现错误时直接调用了 unwrap 来触发 panic
    //     tx.send(val).unwrap();
    //
    //     // 通道和所有权转移
    //     // 发送端通过 send 方法将 val 发送出去，然后我们尝试打印 val，结果发生编译错误：
    //     // error[E0382]: borrow of moved value: `val`
    //     //   --> src\main-channel:41:31
    //     //    |
    //     // 28 |         let val = String::from("hi");
    //     //    |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    //     // ...
    //     // 36 |         tx.send(val).unwrap();
    //     //    |                 --- value moved here
    //     // ...
    //     // 41 |         println!("val is {}", val);
    //     //    |                               ^^^ value borrowed here after move
    //     //
    //     // 假设说在发送完 val 后，我们还可以访问或者使用这个值 val
    //     // 而这个 val 实际上被发送到另外一个线程中
    //     // 那个线程就可以在我们尝试重新使用这个 val 之前修改或丢弃它
    //     // 这些修改极有可能造成不一致或产生原本不存在的数据
    //
    //     // 为了防止上述情况发生，send 函数会获取参数的所有权，并在参数传递时将所有权转移给接收者
    //     // 这可以阻止我们意外地使用已经发送的值
    //     println!("val is {}", val);
    // });
    //
    // // 在主线程中接收消息
    // // 通道的接收端有两个可用于获取消息的方法：recv和try_recv
    // // 我们使用的 recv（也就是 receive 的缩写）会阻塞主线程的执行直到有值被传入通道
    // // 一旦有值被传入通道，recv 就会将它包裹在Result<T, E>中返回
    // // 而如果通道的发送端全部关闭了，recv则会返回一个错误来表明当前通道再也没有可接收的值
    //
    // // try_recv 方法不会阻塞线程，它会立即返回 Result<T, E>：当通道中存在消息时，返回包含该消息的Ok变体；否则便返回Err变体
    // // 当某个线程需要一边等待消息一边完成其他工作时，try_recv 方法会非常有用
    // // 我们可以编写出一个不断调用 try_recv 方法的循环，并在有消息到来时对其进行处理，而在没有消息时执行其他指令
    // let received = rx.recv().unwrap();
    //
    // println!("Got: {}", received);


    // 发送多个值并观察接收者的等待过程

    // thread::spawn(move || {
    //
    //     // 创建了一个用于存储字符串的动态数组
    //     // 在主线程中，我们会将rx视作迭代器，而不再显式地调用recv函
    //     // 数。迭代中的代码会打印出每个接收到的值，并在通道关闭时退出循
    //     // 环
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),String::from("thread"),
    //     ];
    //
    //     for val in vals {
    //         // 我们会迭代动态数组来逐个发送其中的字符串
    //         // 每发送完一个消息，就让新线程暂停 1s
    //         tx.send(val).unwrap();
    //         // 让线程暂停指定的时间
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // // 在主线程中，我们会将rx视作迭代器，而不再显式地调用 recv 函数
    // // 迭代中的代码会打印出每个接收到的值，并在通道关闭时退出循环
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // 预期的输出是每隔 1s，打印出一个值：
    // Got: hi
    // Got: from
    // Got: the
    // Got: thread

    // 事实也正如此，主线程确实是每隔 1s 打印出一个值
    // 我们并没有在主线程的 for 循环中执行暂停或延迟指令，这也就表明主线程确实是在等待接收新线程中传递过来的值

    // 通过克隆发送者创建多个生产者
    // mpsc 是 英 文 “multiple producer, single consumer”（多个生产者，单个消费者）的缩写
    // 我们可以实现多重重生产者的模式：通过克隆通道的发送端来创建出多个能够发送值到同一个接收端的线程

    // 在创建第一个新线程前调用了通道发送端的 clone 方法，这会为我们生成可以传入首个新线程的发送端句柄
    // 随后，我们又将原始的通道发送端传入第二个新线程。这两个线程会各自发送不同的消息到通道的接收端
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];


        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    // 主线程中接收消息
    for received in rx {
        println!("Got: {}", received);
    }

    // 打印结果如下：
    // Got: hi
    // Got: more
    // Got: message
    // Got: from
    // Got: for
    // Got: the
    // Got: thread
    // Got: you

    // 注意：根据你所使用的操作系统的不同，这些字符串的打印顺序也许会有所不同，这是因为系统使用的调度策略不同
}