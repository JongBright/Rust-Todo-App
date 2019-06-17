use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;


struct TodoList {
    todos: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList::_todolist_from_file("/home/muream/todo.txt")
    }
    
    fn _todolist_from_file(path: &str) -> TodoList{
        match File::open(path) {
            Ok(file) => {
                let mut todo_list = TodoList{todos: vec![]};
                for line in BufReader::new(file).lines(){
                    let line = line.unwrap();
                    todo_list._add_item_from_string(&line);
                }

                todo_list
            },
            Err(_) =>{
                println!("Couldn't read the todo file, creating a new todolist.");
                TodoList{todos: vec![]}
            },
        }
    }

    fn _add_item_from_string(&mut self, string: &String){
        let split: Vec<&str> = string.split(" - ").collect();
        let is_checked = if split[0] == "[x]" {true} else {false};
        let text = split[1].to_string();
        self.add_item(text, is_checked);
    }

    fn write_to_file(&self, path: &str) {
        let mut file = File::create(&path).unwrap();
        file.write_all(self.as_string().as_bytes()).unwrap();
    }

    fn as_string(&self) -> String {
        let mut string = String::new();
        for todo in &self.todos{
            string = string + &todo.as_string() + "\n";
        }

        string
    }

    fn add_item(&mut self, text: String, is_checked: bool){
        let new_todo = TodoItem::new(text, is_checked);
        self.todos.push(new_todo);
    }

    fn check_item(&mut self, index: usize){
        let item = &mut self.todos[index];
        item.set_checked()
    }

    fn uncheck_item(&mut self, index: usize){
        let item = &mut self.todos[index];
        item.set_unchecked()
    }

    fn edit_item(&mut self, index: usize, new_text: String){
        let item = &mut self.todos[index];
        item.set_text(new_text)
    }

    fn delete_item(&mut self, index: usize){
        self.todos.remove(index);
    }

    fn print(&self) {
        for (i, item) in self.todos.iter().enumerate(){
            println!("{} {}", i, item)
        }
    }

}


struct TodoItem {
    is_checked: bool,
    text: String,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl TodoItem {
    fn new(text: String, is_checked: bool) -> TodoItem {
        TodoItem {is_checked: is_checked, text: text}
    }
    fn set_checked(&mut self) {
        self.is_checked = true;
    }

    fn set_unchecked(&mut self) {
        self.is_checked = false;
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn as_string(&self) -> String {
        let checked_display = if self.is_checked {"[x]"}  else {"[ ]"};
        format!("{} - {}", checked_display, self.text)
    }
}

fn main() {
    let mut todolist = TodoList::new();

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 1{
        println!("You need to pass some argumets.");
        println!("Use`todo help` to get a list of arguments");
        return;
    }

    let file_path = "/home/muream/todo.txt";

    match arguments[1].as_str() {
        "get" => todolist.print(),
        "add" => {
            todolist.add_item(arguments[2].clone(), false);
            todolist.write_to_file(file_path);
        },
        "delete" =>{
            todolist.delete_item(arguments[2].parse().unwrap());
            todolist.write_to_file(file_path);
        },
        "check" => {
            todolist.check_item(arguments[2].parse().unwrap());
            todolist.write_to_file(file_path);
        },
        "uncheck" => {
            todolist.uncheck_item(arguments[2].parse().unwrap());
            todolist.write_to_file(file_path);
        },
        "edit" => todolist.edit_item(arguments[2].parse().unwrap(), arguments[3].clone()),
        "help" => {
            let help = "
get --> Prints the todo list to the command line.
add \"Some Text\" --> Add a new todo item.
delete <int> --> Delete the todo with the index <int>.
check <int> --> Check the todo with the index <int>.
uncheck <int> --> Uncheck the todo with the index <int>.
edit <int> \"Some Text\" --> Change the text of the todo with the index <int>.
help --> Prints this message.
        ";
            println!("{}", help)
        },
        _ => ()
    };
}
