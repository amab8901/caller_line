This crate lets you obtain the line from which the current function was called.

# Example
```
use caller_line::caller_line;

fn call_this_function() {
    let caller_line = caller_line();

    println!("called from {caller_line}");
}

fn function_wrapper() {
    call_this_function();
}

fn main() {
    function_wrapper();
}

```

The above code will return the following:
```
called from $HOME/project_path/src/main.rs:10
```