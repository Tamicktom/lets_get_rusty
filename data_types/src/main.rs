fn main() {
    // boolean
    let b1: bool = true;

    // unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // singed integers
    let s1: i8 = 1;
    let s2: i16 = 1;
    let s3: i32 = 1;
    let s4: i64 = 1;
    let s5: i128 = 1;

    // floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str and astring
    let c1: char = 'c';
    let c2: &str = "hello";
    let c3: String = String::from("Hello");

    // arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let i1: i32 = a1[4];

    // tuples
    let t1: (i32, i32, i32) = (1, 2, 3);
    let t2: (i32, f64, &str) = (5, 5.0, "5");

    let t3: i32 = t1.1;
    let (t4, t5, t6) = t2;

    let unit = ();

    // Type aliasing
    type Age = u8;

    let a1: Age = 57;
}
