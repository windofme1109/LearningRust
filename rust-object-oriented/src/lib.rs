// Rust - 面向对象编程

// 面向对象三大特性 - 继承 / 封装 / 多态


// 定义一个公开的结构体，外界可以访问这个结构体，但是其内部的字段是私有的，外界不可访问
pub struct AverageCollection {
    // 因为结构体AveragedCollection封装了内部的实现细节，所以我们能够在未来轻松地改变数据结构等内部实现
    // 例如，我们可以在 list 字段上使用 HashSet<i32> 代替 Vec<i32>
    // 只要 add、remove和 average 这几个公共方法的签名保持不变
    // 正在使用 AveragedCollection 的外部代码就无须进行任何修改
    list: Vec<i32>,
    average: f64
}

impl AverageCollection {
     // 对外暴露的方法 - 增加元素
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    // 对外暴露的方法 - 移除元素
    pub fn remove(&mut self) -> Option<i32>{
        let result = self.list.pop();

        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None
        }
    }
    // 对外暴露的方法 - 获得平均值
    pub fn average(&self) -> f64 {
        self.average
    }

    // 内部私有的方法 - 更新平均值
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
