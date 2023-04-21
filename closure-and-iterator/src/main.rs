use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

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


fn generate_workout(intensity: u32, random_number: u32) {

    // 我们希望只调用一次 simulated_expensive_calculation 函数，因为使用这个函数的代价比较大，所以要尽量少的可能调用
    // 可以将 simulated_expensive_calculation 函数的调用放在外面，使用一个变量接收
    // 替换掉原来需要调用 simulated_expensive_calculation 的地方
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        // 在这个条件下，存在两次调用 simulated_expensive_calculation 这个函数的问题
        // 结果是用户会等到比较长的时间
        println!(
            "Today, do {} pushups",
            // simulated_expensive_calculation(intensity)
            expensive_result
        );

        println!(
            "Next, do {} situps",
            // simulated_expensive_calculation(intensity)
            expensive_result
        );
    } else {
        // 
        if random_number == 3 {
            println!("Take a break  today! Remeber to stay hydrated!");
        } else {
            println!(
            "Today, run for {} minutes",
            // simulated_expensive_calculation(intensity)
            expensive_result
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