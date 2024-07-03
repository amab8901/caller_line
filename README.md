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
called from <project path>/src/main.rs:10
```

...where `<project path>` is a placeholder for the project path, and `10` is the line number of `call_this_function();` in the function body of `fn function_wrapper() {...}`.