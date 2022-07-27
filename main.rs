use std::env;
use std::process;
fn main() {
    let tasks = vec!["shital"];
    let args: Vec<String> = env::args().collect();
    if args[1] == String::from("Read") {
        read(tasks);
    } else if args[1] == String::from("Add") {
        let tasks = add(tasks, &args[2]);
    } else if args[1] == String::from("Del") {
        add(tasks, &args[2]);
    }
}

fn read(tasks: Vec<&str>) {
    println!("Tasks are \n ");
    for task in tasks {
        println!(" {} \n", task);
    }
}
fn add<'a>(mut tasks: Vec<&'a str>, tbadd: &'a str) -> Vec<&'a str> {
    println!("Before adding tasks are {:?}", tasks);
    tasks.push(tbadd);
    read(tasks.clone());
    tasks
}
fn del(mut tasks: Vec<&str>, tbdel: &str) {
    println!("Before adding tasks are {:?}", tasks);
    tasks.retain(|&x| x != tbdel);
    read(tasks)
}
