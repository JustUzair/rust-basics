use basics::greet;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::thread;
fn main() {
    //1️⃣---------------------------- 🖨 Print 🖨 ------------------------------
    println!("Hello, world!");

    //2️⃣---------------------------- 🔠 Variables 🔠 ----------------------------
    // @dev variables are immutable by default
    // @notice immutability allows for safety concurrency and speed
    let bunnies = 2; // Immutable - 🚧

    // @dev formatting string in println
    println!("Bunny Value : {}", bunnies);
    // @dev variables can be declared mutable by using the keyword 'mut'
    let mut this_is_immutable_variable = 2; // Mutable - ✅
    this_is_immutable_variable = 10; // this is valid

    //  @dev 'const' can be used to declare a variable stricter version of 'let'
    //  @notice USE SCREAMING CASE 😱
    //  @notice type annotation ":i32" is required ⚠
    //  @notice value to the right must be a constant expression ⚠
    //  @notice since values are initialized at compile time, compilation is faster ⚠
    const WARP_FACTOR: f64 = 9.9;

    //3️⃣---------------------------- Variables Destructuring ----------------------------
    // @dev variables can be initialized by destructuring the data on the right ✅
    let (_var1, _var2) = (1, 2);

    //4️⃣---------------------------- 🔍 Variable Scope 🔍 ----------------------------
    // @notice variables are restricted to the blocks they are declared in 🔳
    let mut this_is_main_block = 10;
    {
        let mut this_is_nested_block = 20;
        // @notice 'this_is_nested_block' & 'this_is_main_block' are accessible here ✅
        // println(this_is_nested_block) Correct - ✅
        // println(this_is_main_block) Correct - ✅
    }
    // @notice 'this_is_nested_block' is not accessible here❌
    // println(this_is_nested_block) Error - ❌
    // println(this_is_main_block) Correct - ✅

    //5️⃣---------------------------- Changing Mutability of Variables & Shadowing of Variables ----------------------------
    let mut x = 5; // x is Mutable - ✅
    let x = x; // x is Immutable - 🚧

    // @dev Shadowing a variable in the same scope
    let value = "YO! What's Up?";
    let value = "Nothing much!";
    println!("Value : {}", value); // Prints "Nothing much!"

    //6️⃣---------------------------- 📝 Memory Safety Feature 📝 ----------------------------

    // @dev Accessing variables before initialization
    /*
       --------------------------------------------------------------------------------------------------------

        Example1️⃣ @notice this block won't work ❌
        let enigma: i32;
        println!("{}",enigma); Value is not Initialized.

       --------------------------------------------------------------------------------------------------------

        Example2️⃣ @notice so will not this block ❌
        let enigma: i32;
        if true{
            engima=505; // value is initialized here
        }
        println!("{}",enigma); Error - ❌
        @notice this will not work because the compiler doesn't know the value of
        true until runtime, so it is not sure about the value of "enigma".
        Since there is no assurance that the value of enigma will be set,
        therefore print line results in an error

        @notice conditional evaluations are handled at runtime, if there is no guarantee that
        the value will be set.

        @notice here guarantee means an "else" block following the "if"

       --------------------------------------------------------------------------------------------------------
        Example3️⃣ @notice this block works ✅

        let enigma: i32;
        if true{
            engima=505;
        }
        else{
            enigma=12; // if the "if" statement fails, value will be set in else block.
        }

        @notice the print statement works, because the else block guarantees that the value of the
        variable will be set

        println!("{}",enigma); Prints Value of "enigma" ✅

    */

    //7️⃣---------------------------- 🟧 Function 🟧 ----------------------------
    /*
        @notice keyword "fn" is used


    */
    fn this_is_a_function() {} // Simple Function
                               //@dev -> f64 depicts the expected return value by the function
    fn this_is_parameterized(length: f64, breadth: f64) -> f64 {
        // return  length+breadth; // returning value explicitly
        length * breadth // same as returning explicitly
    }

    //8️⃣---------------------------- 👨‍🏫 Module & It's uses 👨‍🏫 ----------------------------
    basics::greet(); // calls the function from module
                     // @notice writing absolute path for the file on the left is not feasible
                     // (REFER Line1️⃣)
    greet(); // this works too

    //9️⃣---------------------------- 🌀 Scalar Types 🌀 ----------------------------
    /*
        1. Integer
            > unsigned
                > u8
                > u16
                > u32
                > u64
                > u128
                > usize
            > signed
                > i8
                > i16
                > i32   --> default, because it is fastest in comparison to other
                > i64
                > i128
                > isize --> size represents platform's pointer type
        2. Float
            //@notice .1 is invalid but 0.1 is valid, a number must be present before a "."
            > f32   --> 32 bits precision
            > f64   --> 64 bits precision (default)

        3. Character
            //@notice keyword char is used
            //@notice size of char is 4 bytes (32bits)
            //@notice single quotes are to be used to declare character

                > let my_rocket = '🚀'

        4. Boolean
            //@notice keyword bool is used

            //@dev type casting boolean to integer can be done as follows
            //true as u8
            //false as u8


    */

    //🔟---------------------------- 💹 Compound Types 💹 ----------------------------
    // @notice compound types, bind together different types into a single one

    // 1. Tuple (can have maximum of 12 elements)
    let tuple: (u8, f32, i32) = (2, 3.14, 23);
    println!("VALUE 1 : {}", tuple.0);
    println!("VALUE 2 : {}", tuple.1);
    println!("VALUE 3 : {}", tuple.2);
    // destructuring a tuple
    let (jaz, buzz, phil) = tuple;
    println!("{},{},{}", jaz, buzz, phil);

    // 2. Arrays  (can have maximum of 32 elements)
    let buf: [u8; 3] = [5, 2, 4]; // [u8;3] determines, three values of u8 type
    let buf2 = [1, 2, 4];

    println!("Accessing index 0 in array: {}", buf[0]);

    //1️⃣1️⃣---------------------------- 🎮 Control Flow 🎮 ----------------------------
    let num = 32;
    // Way 1 of assigning values in if statement
    let mut msg;
    if num == 5 {
        msg = "five";
    } else if num == 32 {
        msg = "thirty-two"
    } else {
        msg = "meh!!"
    }
    // Way 2 of assigning values in if statement
    let msg2 = if num == 5 {
        "five"
    } else if num == 32 {
        "thirty-two"
    } else {
        "meh!!"
    };

    // Unconditional Loops
    loop {
        break;
    }

    // Unconditional Loops with labels
    'outermost: loop {
        loop {
            loop {
                break 'outermost; // breaks the loop with label 'outermost
            }
        }
    }

    /*
        'outermost:loop{
            loop{

                continue 'outermost;
            }
        }
    */

    // While loop
    // while healthy(){}

    // @notice we can also create while loop using an unconditional loop
    /*
        loop{
            if !healthy() {break;}
            // do stuff
        }
    */

    // For Loop
    for num in [25, 12, 5].iter() { // .iter() iterates over the array
         // do stuff
    }

    for (key, value) in [(1, 2), (5, 45), (6, 12)].iter() {
        println!("Key:{} ; Value:{}", key, value);
    }

    for num in 0..5 {
        // start..end --> start is inclusive and end is exclusive i.e prints value from 0 to 4
        println!("Current Range : {}", num);
    }
    println!("----------------------------");
    for num in 0..=5 {
        // start..=end --> start and end are both inclusive i.e prints value from 0 to 5
        println!("Current Range : {}", num);
    }

    //1️⃣2️⃣---------------------------- 🔤 Strings 🔤 ----------------------------
    // @notice data of "string slice" cannot be modified
    // @notice data of "String" can be modified
    let string_slice = "This is slice";
    let mut string_type = "String".to_string();
    string_type.insert(5, 'm'); // changing the string works for "String type" ✅

    let byte_string = string_type.bytes(); // returns byte array

    //1️⃣3️⃣---------------------------- 🆔 Ownership of Variables 🆔 ----------------------------
    // 📧 The 3 Rules of Ownership of Variables📧
    //          > Each value has an owner
    //          > Only one owner (owned variables can be borrowed)
    //          > Value of variable is dropped when owner goes out of scope

    // @notice example of ownership in rust
    let s1 = String::from("Ownership string");
    let s2 = s1;
    // println!("{}","s1"); Error ❌, because value of s1 is moved into "s2"
    // @notice the error is thrown because rust, un-initializes the value of "s1" because 2 variables own the value. 📧
    // @notice if the "s1" is not un-initialized then the memory-safety aspect of rust would fail ❌

    // @notice now we "copy" the value of s1 and not move the ownership
    let s3 = s2.clone(); // s2 is used because currently "s2" is the owner of the string from "s1" 📧

    //1️⃣4️⃣---------------------------- 🏦 References and Borrowing 🏦 ----------------------------
    // @notice when we want to use the value of owned variables into some function, remember rule 3. of ownership,
    // the value of variable is dropped, therefore we pass the values as references
    let s1 = String::from("Ownership string");
    let mut s2: String = s1.clone();
    fn do_stuff(s: &String) {
        println!("Inside function scope : {}", s);
    }
    fn do_mut_stuff(s: &mut String) {
        s.insert_str(0, "MUTATING...");
        println!("Before mutating by reference : {}", s);
        println!("Mutate by passing reference inside function scope : {}", s);

        // @notice to assign new value to actual value, we need to de-reference the referenced like
        // *s = String::from("REPLACED!!🏃‍♂️");
    }
    do_stuff(&s1); // reference of string "s1" is passed to the function
    do_mut_stuff(&mut s2);
    println!("Outside function scope : {}", s1);
    println!("Mutated s2 outside function scope : {}", s2); // updated string is displayed here

    fn test(s: &mut String, s2: &mut String) {
        println!("{},{}", s, s2);
    }
    // @notice Summary of Referencing and Borrowing
    /*
       > At any given time you can have INFINITE number of immutable references to a variable ♾
       > At any given time you can have ONLY ONE mutable reference to a variable 1️⃣

    */

    //1️⃣5️⃣---------------------------- 👷‍♂️ Structs 👷‍♂️ ----------------------------

    // @notice "impl" keyword is used to define a struct

    // 📧 First we define the structure of "struct" 📧
    struct Cat {
        name: String,
        breed: String,
    }

    // 📧 Then we define the implementation for "struct" 📧

    impl Cat {
        fn new() -> Self {
            Self {
                name: String::from("neko"),
                breed: String::from("persian"),
            }
        }
    }

    // 📧 Finally using the struct 📧
    let persian_cat = Cat::new();
    println!("Cat name : {}", persian_cat.name);
    println!("Cat breed : {}", persian_cat.breed);

    //1️⃣6️⃣---------------------------- 😎 Traits 😎 ----------------------------
    // @notice trait is like "interfaces" or "blueprints" in other programming languages, this also solves the inheritance problem
    trait Sound {
        fn make_sound(&self) -> &str;
    }
    impl Sound for Cat {
        fn make_sound(&self) -> &str {
            "MEOW...🐱"
        }
    }

    //1️⃣7️⃣---------------------------- 📦 Collections 📦 ----------------------------

    // 1. Vectors 📏
    //  Vec<T>
    let mut my_vector: Vec<i32> = Vec::new();
    my_vector.push(10);
    my_vector.push(20);
    my_vector.push(30);

    let x = my_vector.pop();
    println!("{}", my_vector[1]);

    // Creating vectors from literal values (another way of defining vectors)
    let mut my_vector_two = vec![12, 23, 58];

    // 2. HashMaps 🗺
    // HashMap<K,V>
    let mut my_hashmap: HashMap<&str, bool> = HashMap::new();
    my_hashmap.insert("val", true);
    let hifi = my_hashmap.remove("val").unwrap();

    // Some other collections
    // > VecDeque
    // > HashSet
    // > LinkedList
    // > BinaryHeap
    // > BTreeMap
    // > BTreeSet

    //1️⃣8️⃣---------------------------- 🧳 Enums 🧳 ----------------------------
    // @notice keyword "enum" is used

    // Basic Enum Example
    enum Color {
        Red,
        Green,
        Blue,
    }

    // More advanced use case of enum
    enum Dispenser {
        Empty,
        Things(String, i32),
        Place { x: i32, y: i32 },
    }

    // Using the enums
    use Dispenser::*;
    let item = Empty;
    let item2 = Things(String::from("Can"), 32);
    let item3 = Place { x: 25, y: 10 };

    // @notice we can also implement functions for enum

    impl Dispenser {
        fn display(&self) {
            println!("Dispenser Enum function")
        }
    }

    // > The Option Enum
    /*
        enum Option<T> {
            Some(T),
            None,
        }
    */

    let mut none_option: Option<i32> = Option::None;
    println!("{}", none_option.is_some()); // False
    println!("{}", none_option.is_none()); // True
    none_option = Option::Some(25);
    println!("{}", none_option.is_some()); // True
    println!("{}", none_option.is_none()); // False`

    // > The Result Enum
    /*
        #[must_use]    -->  Should be somehow used
        enum Result<T,E>{
            Ok(T),
            Err(E)
        }
    */
    File::open("foo");

    /* File named foo may or may not exist, this is the warning the compiler gives while compilation


       = note: this `Result` may be an `Err` variant, which should be handled
       = note: `#[warn(unused_must_use)]` on by default
       help: use `let _ = ...` to ignore the resulting value


    */

    let res = File::open("foo");
    // Handling the file read (METHOD - 1)
    // let f = res.unwrap();

    // Handling the file read (METHOD - 2)
    // if res.is_ok() {
    //     let f = res.unwrap(); // only unwrap when result is ok
    // }

    // Handling the file read (METHOD - 3)
    match res {
        Ok(f) => {
            // on success do something
            println!("File reading successful");
        }
        Err(e) => {
            // on error do something
            println!("Error while file read");
        }
    }

    //1️⃣9️⃣---------------------------- 🔒 Closures 🔒 ----------------------------
    // @notice closure is an anonymous function that can borrow or capture the data from the scope it is nested in

    /*
        Syntax:
        |x,y| {x+y}
    */
    let add = |x, y| x + y;

    println!("{}", add(1, 2)); // add returns 3

    // Proof that closure borrows the variable from the scope
    let s = String::from("🍓");
    let f = move || {
        // moves the value of s into f
        println!("{}", s);
    };
    f();

    // Another great use-case of closures
    let mut vector = vec![12, 23, 45];
    let value = vector
        .iter()
        .map(|x| x * 3) // multiply each value by 3
        .filter(|x| *x > 10) // filter values greater than 10
        .fold(0, |sum, x| sum + x); // add the values with initial sum = 0

    println!("Closure vector value : {}", value);

    //2️⃣0️⃣---------------------------- 🧵 Thread 🧵 ----------------------------
    let handle = thread::spawn(move || {
        // do stuff in child thread
    });
    // do something simultaneously in main thread

    // wait until the thread has exited
    handle.join().unwrap();
}
