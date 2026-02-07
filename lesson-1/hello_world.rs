fn main() {
    println!("Hello, world!");

    // Binding and mutability
    // A variable can be used only when it is initialized
    //let x: i32; // Uninitialized and used, an Error!
    //let y: i32; // Uninitialized and not used, a warning.
    //assert_eq!(x, 5); //Error and it will crash because x is not initialized
    //println!("Failed");

    // In rust a variable is by default immutable
    // a variable must be explicitly declared as mutable
    /*let mut x: i32 = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("Success");
    */

    // Scope is the range within the program for which the item is valid
    /*let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("x: {} | y: {}", x, y);
    }
    println!("x: {} | y: {}", x, y); //It will display an error if y was declared within the scope of the curly braces
    */

    define_x();

    // Shadowing in rust allows a variable to be declared a second time to behave as a shadow of the original variable
    /*let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);  // This is a different scope with a different value
        println!("inner scope x: {}", x);
    }

    assert_eq!(x, 5);   // This is outside the second scope in the original scope
    println!("outer scope x: {}", x);
    */

    /*let x = 42;
    println!("Third declaration outside scope x: {}", x);

    // #6
    let mut a: i32 = 1;
    a = 7;
    // Shadowing and rebinding
    //let mut a = a; // 7
    a += 3;
    println!("a: {}", a);
    */

    /*let y: i32 = 5;
    // Shadowing and re-binding to a string when original type was i32 int
    let y: &str = "Rebinding Y to a string!";
    println!("{}", y);
    println!("Success!");
    */

    // #7 declaring a variable and not using it displays a warning when compiling
    // let unused: i32 = 1;

    // #8 we can use a pattern with let to destructure a variable in a tuple to separate variables
    // we can use shadowing and mutable types
    /*let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
    */

    // # 9 Destructuring assignments
    // In Rust 1.59: You can use tuples, slice and struct patterns as the left hand side of an assignment
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    println!("Success!");


}

fn define_x() {
    let x: &str = "hello";
    println!("{}, universe!", x);
}