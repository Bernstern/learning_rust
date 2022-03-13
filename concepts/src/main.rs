fn main() {
    println!("This is a simple example showing how mutability works:");
    // Trying to compile this throws an error because x is not defined to be mutable, default in the language is immutable
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // Sometimes it is more performant to modify large data structures in place than it is to copy and return newly alloc'd instances.

    println!("This is all about how constants work:");
    // Constants cannot change like default variables, however you can never use mut
    const THREE_MINUTES_IN_SECONDS: u32 = 60 * 3;
    // Check out https://doc.rust-lang.org/reference/const_eval.html for what can be used in a const at compile time
    println!("Did you know that three minutes is {} seconds?", THREE_MINUTES_IN_SECONDS);

    println!("This is about shadowing:");
    // You can declare a new variable with the same name as an older variable in the scope
    let x = 5;
    // New and cooler x replaces old x
    let x = x + 1;
    // The shadowing only persists in this tighter scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // Then returns afterwards
    println!("The value of x is: {}", x);

    let x = "This is now a string!";
    println!("You can even change the type of x while shadowing: {}", x);

    println!("Let's talk about typing!");
    // Rust can usually infer types at compile time but its a solid practice to type everything

    // There are four scalar types in Rust: Ints, Floats, Bools, and Chars

    // Keep in mind that when compiling at release time, Rust will wrap ints without warning (CTF!)
    // There are methods wrapping, checked, overflowing and saturating to check int math
    let x: u8 = 255;
    let x: u8 = (x).saturating_add(255);
    // let y: u8 = 255 + 255; <= This line will result in a compiler error in debug mode but not release
    println!("Using something like saturing add will max the value out ({})!", x);

    // Float example
    println!("Floats are pretty self explanatory {}", 2.0);

    // Compound types like tupoles are a little fancier, they have fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}, you can also use indices like tup.0 {}", y, tup.0);

    // Arrays in rust must have the same type and have a fixed length
    let a: [i32; 5] = [1,2,3,4,5];

    // Functions are similar to other languages, using snake case for naming
    another_func();
    function_with_params(5);

    // Loops in rust are freaking sick because you can label them and break out of them
    let mut count = 0;
    'counting_loop: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // You can also return values from loops (think like checking if a thread exited)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is: {}", result);

    // Ownership example
    ownership();
}

fn another_func(){
    println!("This is another function");
}


fn function_with_params(x: i32) -> i32 {
    // This line below is a statement because it ends in a semicolon
    println!("The first param is {}", x);
    x+5 // This is how you return at the bottom of a function, they are preceeded by statement but end in an expression
}

fn ownership() {
    // Every value in Rust has a variable that is its owner, there can only be one owner at a time, when the owner is out of scope, the value is dropped


    // How data is managed on the heap is kinda shown here
    {
        // s1 creates a struct on the stack that points to the string on the heap
        let s1 = String::from("hello");
        // Right here s2 creates a second struct on the stack that points to the same string on the heap
        // But also after this s1 is no longer valid to ensure memory safety
        let s2 = s1;

        // We see this that the following line would throw a compiler error
        // println!("{}, world!", s1);

        // s1 was moved into s2
        println!("{}, world!", s2);
    }

    // You can do a clone operation instead of a move to deeply copy heap data
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        // For variables that just has stack data they will use the copy trait instead of clone
        let x = 5;
        let y = x;
        println!("x = {} y = {}", x, y);
        // If a class implements copy a variable is still valid after assignment to another variable
    }

    {
        let s = String::from("hello");  // s comes into scope
        takes_ownership(s);             // s's value moves into the function and so is no longer valid here


        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
        println!("Hey check me out, I can still be used: {}", x);

        fn takes_ownership(some_string: String) { // some_string comes into scope
            println!("{}", some_string);
        } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

        fn makes_copy(some_integer: i32) { // some_integer comes into scope
            println!("{}", some_integer);
        } // Here, some_integer goes out of scope. Nothing special happens.
    }

    // Borrowing: We can use references to avoid returning variables we pass into functions and re-assigning them
    {
        let s1 = String::from("Test");
        // The &s1 syntax creates a reference to the value s1 which has no owner, bc there is no owner, the value it points to will
        // not be dropped when the ref goes out of scope
        let len = calculate_len(&s1);
        println!("The length of '{}' is {}.", s1, len);

        fn calculate_len(s: &String) -> usize {
            s.len()
        }
    }

    // Mutable references allow modifying a borrowed variable but you can only have one mut ref to a piece of data at a time
    {
        let mut s = String::from("Hello");
        change(&mut s);

        fn change(str: &mut String) {
            str.push_str(" World");
        }
    }
}