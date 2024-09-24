fn main() {
    println!("Scalar Types:");
    println!("This is an unsigned integer: `let x: u32 = 5`");
    let x: u32 = 5;
    println!("{x}");
    println!("This is a signed integer: `let y: i32 = -5`");
    let y: i32 = -5;
    println!("{y}");
    println!(
        r#"
    You can notate integers with many notations. 
    Please see Table 3-2 on this page from the rust book:
    https://doc.rust-lang.org/book/ch03-02-data-types.html
    "#
    );
    println!("This is a float: `let z: f64 = 7.35`");
    let z: f64 = 7.35;
    println!("{z}");
    println!("This is a bool: `let a: bool = false`");
    let a: bool = false;
    println!("{a}");
    println!("These are examples of chars");
    println!(
        r#"
    `let c = 'z';`
    `let z: char = 'ℤ';`
    `let heart_eyed_cat = '😻';`
    "#
    );
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");
    println!("Let's look at Compound Types!");
    println!("This is a tuple: `let tup: (i32, f64, u8) = (500, 6.4, 1);`");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        r#"
    You can use the tuple to define variables like this
    `let (b, c, d) = tup;`    
    "#
    );
    let (b, c, d) = tup;
    println!("{b} {c} {d}");
}
