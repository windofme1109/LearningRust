use::std::collections::HashMap;
fn main() {
    // HashMap<K, V>
    // HashMap 在内部实现中使用了哈希函数，这同时决定了它在内存中存储键值对的方式
    // 许多编程语言都支持这种类型的数据结构，只是使用了不同的名字
    // 例如:哈希(hash)、映射(map)、对象(object)、哈希表 (hash table)、字典(dictionary)或关联数组(associative array)等

    // 如果我们想存储一些形式为键值对的数据，其中键为指定的类型，如 String
    // 显然动态数组不能满足我们的需求，因为动态数组是的索引是数字形式的
    // 在这种情况下可以使用 hashmap
    
    // 创建新的 hashmap
    // hashmap 的使用频率相比于 String 和动态数组低一些
    // 所以它没有被包含在预导入模块内，需要手动导入 hashmap
    let mut scores = HashMap::new();
    // 向 hashmap 中插入元素
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 标准库对 hashmap 的支持也不如 String 和 Vector，例如它没有提供一个可以用于构建 hashmap 的内置宏。

    // 和动态数组一样，hashmap 也将其数据存储在堆上。上面例子中 的 HashMap 拥有类型为 String 的键，以及类型为i32的值
    // 依然和动态数组一样，hashmap 也是同质的:它要求所有的键必须拥有相同的类型，所有的值也必须拥有相同的类型

    // 使用 zip 方法创建 hashmap
    // 在一个由键值对组成的元组动态数组上使用 collect 方法。
    // 这里的collect方法可以将数据收集到很多数据结构中，这些数据结构也包括 HashMap
    // 例如，假设我们在两个不同的动态数组里分别存储了队伍的名字和分数，那
    // 么我们就可以使 用zip方法来创建一个元组的数组，其中第一个元组由"Blue"与10组 成，
    // 以此类推。接着，我们还可以使用 collect 方法来将动态数组转换为 hashmap
    let team = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];
    let score2: HashMap<_, _> = team.iter().zip(initial_score).collect();
    // 注意，我们指定了 score2 的类型为 HashMap<_, _>，这个类型标记不能省略
    // 因为 collect 可以作用于许多不同的数据结构
    // 如果不指明类型的话，Rust就无法知道我们具体想要的类型
    // 但是对于键值的类型参数，我们则使用了下画线占位
    // 因为 Rust 能够根据动态数组中的数据类型来推导出 hashmap 所包含的类型

    // hashmap 与所有权
    // 对于那些实现了 Copy trait 的类型，例如i32，它们的值会被简单地复制到 hashmap 中
    // 而对于 String 这种持有所有权的值，其值将会转移（move）且所有权会转移给 hashmap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 的所有权已经转移到 hashmap 里，因此从这里无法访问 field_name 和 field_value 
    // error[E0382]: borrow of moved value: `field_name`
    // error[E0382]: borrow of moved value: `field_value`
    // println!("{}, {}", field_name, field_value);
    // 假如我们只是将值的引用插入 hashmap，那么这些值是不会被移动到 hashmap 中的
    // 这些引用所指向的值必须要保证，在 hashmap 有效时自己也是有效的
    
    // 访问 hashmap 中的值
    let team_name = String::from("Blue");
    let mut scores2 = HashMap::new();
    // 向 hashmap 中插入元素
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Yellow"), 50);

    // get 方法返回的是 Option 枚举
    // 可以通过 team_name 在 scores2 这个 hashmap 中找到相关的值
    // blue_score 的结果是 Some(&10) 这个变体
    // 这个 hashmap 中没有键所对应的值，那么 get 就会返回 None
    let blue_score = score2.get(&team_name);

    match blue_score {
        Some(value) => println!("val is {}", value),
        None => ()
    }

    // 遍历一个 hashmap
    // 类似于动态数组，我们同样可以使用一个 for 循环来遍历 hashmap 中所有的键值对
    // 遍历的顺序和我们插入的顺序不一定是一致的，也就是说，hashmap 内部存的数据，是无序的
    for (k, v) in &scores2 {
        // Blue: 10
        // Yellow: 50
        println!("{}: {}", k, v);
    }

    // 更新 hashmap 中的值
    // 1. 覆盖旧值
    //    将一个键值对插入哈希映射后，
    //    接着使用同样的键并配以不同的值来继续插入，之前的键所关联的值就会被替换掉
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 25);
    scores3.insert(String::from("Blue"), 100);
    // {"Blue": 100}
    println!("{:?}", scores3);

    // 2. 只在键没有对应值时插入数据
    //    在实际工作中，我们常常需要检测一个键是否存在对应值，如果不存在，则为它插入一个值。
    //    哈希映射中提供了一个被称为 entry 的专用 API 来处理这种情形
    //    它接收我们想要检测的键作为参数
    //    并返回一个叫作 Entry 的枚举作为结果，这个枚举指明了键所对应的值是否存在
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);
    scores4.entry(String::from("Yellow")).or_insert(50);
    
    scores4.entry(String::from("Blue")).or_insert(50);
    // {"Yellow": 50, "Blue": 10}
    println!("{:?}", scores4);
    // Entry 枚举的 or_insert 方法被定义为返回一个 Entry 键所指向值的可变引用
    // 假如这个值不存在，就将参数作为新值插入 hashmap 中
    // 并把这个新值的可变引用返回。
    // 使用这个功能要比我们自己编写逻辑代码更加简单，使代码更加整洁，另外也可以与借用检查器结合得更好

    // 因为 Yellow 这个 key 不存在，所以我们插入了这个键值对
    // 因为 Blue 这个 key 存在，所以不会调用 or_insert 更新

    // 3. 基于已有的值进行更新
    //    哈希映射的另外一个常见用法是查找某个键所对应的值，并基于这个值来进行更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Entry 的 or_insert 方法，如果指定的键对应的值不存在，那么将 or_insert 接收的参数插入，这个参数就变成了相应的 value
        // 然后 or_insert 方法将这个 value 的可变引用返回
        // 可变引用进而被存储到变量 count 上
        let count =  ap.entry(word).or_insert(0);
        // 为了对这个值进行赋值操作，我们必须首先使用星号(*)来对 count 进行解引用
        *count += 1; 
    }
    // {"hello": 1, "world": 2, "wonderful": 1}
    println!("{:?}", map);
    // 上面的代码用于计算一段文本中每个单词所出现的次数
    // 使用了一个以单词作为键的哈希映射来记录它们所出现的次数
    // 在遍历的过程中，假如出现了一个新的单词，我们就先将值 0 插入 hashmap 中

}