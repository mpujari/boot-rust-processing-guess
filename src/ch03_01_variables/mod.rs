// Learning from https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

pub mod ch03_01_variables_mod {

    const THREAD: i32 = 373737;

    pub fn code_with_variables() {
        // Variables are immutable by default
        let x: i32 = 5;

        println!("The value of x is: {}", x);

        // Variables are mutable with the mut keyword
        let mut y: i32 = 5;
        println!("The value of y is: {}", y);
        y = 6;
        println!("The value of y is: {}", y);

        // Constants are always immutable
        const MAX_POINTS: u32 = 100_000;
        println!("The value of MAX_POINTS is: {}", MAX_POINTS);

        println!("Const is {}", THREAD);
    }

    pub fn code_with_shadowing() {
        let x = 5;
        println!("X: {}", x);
        {
            let x = 6;
            println!("X value in block: {}", x);
        }
        println!("X value in after the block: {}", x);
    }
}
