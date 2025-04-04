struct SystemLog {
    level: String,
    message: String,
}

pub trait Renderable {
    fn render(&self) -> String;
}

impl Renderable for SystemLog {
    fn render(&self) -> String {
        format!("{}: {}", self.level, self.message)
    }
}

fn print_all<T: Renderable>(list: &[T]) {
    for elem in list {
        println!("{}", elem.render());   
    }
}
fn main() {

    let sys_log = SystemLog{ level: "ERROR".to_string(), message: "This is broken".to_string()};
    let mut logs = Vec::new();
    logs.push(sys_log);
    print_all(&logs);


    println!("Hello, world!");
}
