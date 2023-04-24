use rand;

pub fn add_one(x: i32) -> i32 {
     x + 1
}

pub fn add(x: i32, y: i32) -> i32 {
     x + y
}


// 添加测试
# [cfg(test)]
mod test {
     use super::*;
     #[test]
     fn it_works() {
          assert_eq!(add_one(2), 3);
     }
}