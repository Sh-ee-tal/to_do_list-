use std::env;

use std::process;
fn main() {
    let mut to_do = Tasks {
        tasks: vec!["shital"],
        args: env::args().collect(),
    };
    to_do.change();
}
struct Tasks<'a> {
    tasks: Vec<&'a str>,
    args: Vec<'a String>,
}
impl<'a> Tasks<'a> {
    fn change(&self) {
        if self.args[1] == "Read" {
            read(&self.tasks);
        } else if self.args[1] == "Add" {
            add(&mut self.tasks, &self.args[2]);
        } else if self.args[1] == "Del" {
            del(&mut self.tasks, &self.args[2]);
        }
    }
}

fn read(tasks: &Vec<&str>) {
    println!("Tasks are \n ");
    for task in tasks {
        println!(" {} \n", task);
    }
}
fn add<'a>(tasks: &mut Vec<&'a str>, tbadd: &'a str) {
    println!("Before adding tasks are {:?}", tasks);
    tasks.push(tbadd);
    read(tasks);
}
fn del(tasks: &mut Vec<&str>, tbdel: &str) {
    println!("Before adding tasks are {:?}", tasks);
    tasks.retain(|&x| x != tbdel);
    read(tasks)
}
