// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

// 使用下述命令执行
// rustc src/16-mod-layout/split.rs && ./split

// 文件夹、文件都是 mod 路径中的一部分，
// 对于文件全局作用域中的名字，其为 文件名mod 的一部分
// 对于文件 mod 中的名字，气味 文件名mod.mod 的一部分

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

