fn main(){
    println!("Hello, HELL!");

    //Why println! is a Macro

    //There are multiple reasons why println! is a macro rather than a function, and we haven’t really explained Rust yet, so it’s not exactly obvious. Here are the reasons:

      //  The string passed to println! can have formatting specifiers in it, and those are checked at compile-time.
      //  Rust functions can only have a fixed number of arguments, but println! (and macros generally) can take a variable number.
      //  The formatting specifiers can have named arguments, which Rust functions cannot.
      //  It implicitly takes its arguments by reference even when they’re passed by value.

    //If none of this makes sense, don’t worry about it. We’ll cover these concepts in more detail later.

}
