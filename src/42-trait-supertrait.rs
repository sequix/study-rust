trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct A {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Person for A {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for A {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for A {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for A {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

// 引用 trait 不加 dyn，是 rust 之前的语法，现在已经 deprecated
// 参考：https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html
// 所以不只是只在返回值大小不确定时加 dyn，而是大小不确定都加 dyn
// fn comp_sci_student_greeting(student: &CompSciStudent) -> String {
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
    let me = A {
        name: String::from("sequix"),
        university: String::from("sau"),
        fav_language: String::from("rust"),
        git_username: String::from("sequix"),
    };
    comp_sci_student_greeting(&me);
}
