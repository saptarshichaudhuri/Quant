//The main program is always the first line of code
//that executes in every executable rust program
fn main(){
    //println! calls a Rust macro
    //if it was function, it would not include the "!"
    println!("Hello, world!");
    //the semi-colon ; indicates that the current expression
    //is over and the next one is ready to begin
}

//Execution process
/*
1. rustc main.rs -> this compiles the program
2. after compiling succesfully rust outputs a binary executable
3. Rust is an ahead of time compiled language meaning we can
compile and give the executable to someone else to execute

*/