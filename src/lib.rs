// @notice lib.rs is the default 

// @notice the function in the library are private 
// by default even to the file within the same project

// @dev "pub" is used to declare the function as public
pub fn greet(){ 
    println!("Hello from module");
}