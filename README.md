This is a simple CLI todo app I made to get familiar with Rust.

it works like so:

`todo get` --> Prints the todo list to the command line.  
`todo add \"Some Text\"` --> Add a new todo item.  
`todo delete <int>` --> Delete the todo with the index <int>.  
`todo check <int>` --> Check the todo with the index <int>.  
`todo uncheck <int>` --> Uncheck the todo with the index <int>.  
`todo edit <int> \"Some Text\"` --> Change the text of the todo with the index <int>.  
`todo help` --> Prints this message.  

When compiling the project directly, simply replace `todo` by `cargo run`
