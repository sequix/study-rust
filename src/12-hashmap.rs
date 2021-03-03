use std::collections::HashMap;

fn main() {
    // 对于实现 Copy trait 的类型，如 i32，这些值被复制到哈希映射中。对于 String 这样的所有值，这些值将被移动，散列映射将成为这些值的所有者，如下：
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 下述语句报错
    // println!("{}: {}", field_name, field_value);

    // HashMap 会拿走 key 的 ownership
    let mut br = HashMap::new();

    br.insert("a".to_string(), "b".to_string());
    br.insert("b".to_string(), "c".to_string());

    // overwrite
    br.insert("b".to_string(), "xxx".to_string());

    // insert only if not exists
    br.insert("c".to_string(), "d".to_string());
    br.entry("c".to_string()).or_insert("new c".to_string());

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !br.contains_key("a") {
        println!("br len {}, but no `a`", br.len());
    }

    br.remove("a");
    println!("br len {}", br.len());

    let keys = ["a", "b"];
    for &k in &keys {
        match br.get(k) {
            Some(v) => println!("{}: {}", k, v),
            None => println!("{} is not in map", k),
        };
    }
    // panic if not found
    // println!("{}: {}", "a", br["a"]);

    for (k, v) in &br {
        println!("2 {}: {}", k, v);
    }

    // inited fixed map
    // let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
    //     .iter()
    //     .cloned()
    //     .collect();

    let timber_resources: HashMap<_, _> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();

    for (k, v) in &timber_resources {
        println!("timber_resource {}: {}", k, v);
    }

    // customized key
    let mut vikings = HashMap::new();

    vikings.insert(Viking::new("Einar", "Denmark"), 25);
    vikings.insert(Viking::new("Olaf", "Norway"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}
