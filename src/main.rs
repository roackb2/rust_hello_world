use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    play_around_primitives();
    play_around_compound_types();
    test_statement();
    let c = simple_add(5, 8);
    println!("The result is {}.", c);
    control_flow();
    ownership();
    reference();
    slice_type();

    let mut input = String::new();

    println!("Should we play the guessing game? \
        type 'play' to play. \
    ");

    io::stdin()
        .read_line(&mut input)
        .expect("Fail to parse input");

    match input.trim() {
        "play" => guessing_game(),
        _ => println!("Alright, don't play")
    };
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        println!("Please input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        println!("You input the guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Nanah, numbers only.");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}

fn play_around_primitives() {
    let mut some_num = 5;
    println!("Some number: {}", some_num);
    some_num = 6;
    println!("Some number is changed to {}", some_num);
    let some_num: u32 = 32;
    println!("Shadowing some number to {}", some_num);
    // unsigned numbers could not be negative
    // let some_num = -64;
    let some_num = 23_987;
    println!("Decimal representation of 23_98: {}", some_num);
    let some_num = 0xab;
    println!("Hex representation of 0xab: {}", some_num);
    let some_num = 0o32;
    println!("Octal representation of 0o32: {}", some_num);
    let some_num = 0b1001_0100;
    println!("Binary representation of 0b1001_0100: {}", some_num);
    let some_num: f32 = 28.97;
    println!("A floating point number: {}", some_num);
    let txt = "😎";
    println!("A char could be emoji: {}", txt);
    let txt = "我是中文";
    println!("Chinese is also fine: {}", txt);
}

fn play_around_compound_types() {
    let tup: (u32, i64, bool) = (28, -12, false);
    println!("A tuple with type (u32, char, bool): {}, {}, {}", tup.0, tup.1, tup.2);
    let (a, b, c) = tup;
    println!("Tuple elements could also be destructured: {}, {}, {}", a, b, c);
    let arr: [u32; 3] = [2, 5, 8];
    println!("Array is of fixed length: [{}, {}, {}]", arr[0], arr[1], arr[2]);
    let arr = [5; 3];
    println!("Array initialization: [{}, {}, {}]", arr[0], arr[1], arr[2]);
}

fn test_statement() {
    let x = {
        let y = 5;
        y + 1 // note that no semicolon after an expression
    };
    println!("Value of x: {}", x);
}

fn simple_add(a: i32, b: i32) -> i32 {
    println!("We're adding {} and {} with our simple_add function.", a, b);
    a + b
}

fn control_flow() {
    let cond = 5;
    // NOTE: No parenthesis needed for conditions
    let res = if cond < 8 {
        println!("Number is less than 8");
        true
    } else {
        println!("Number is greater than 8");
        false
    };
    println!("Assignment of the result for if-else expression: {}", res);
    let res = if cond < 8 { "Number is less than 8" } else { "Number is greater than 8" };
    println!("Assignment of single line if-else expression: {}", res);

    let mut x = 0;
    // NOTE: Rust support loop labeling
    let (x, y) = 'outer_loop: loop {
        x = x + 1;
        println!("value of x: {}", x);
        let mut y = 0;
        loop {
            y = y + 1;
            println!("value of y: {}", y);
            if x > 30 {
                println!("break outer loop if x > 30");
                break 'outer_loop (x, y);
            }
            if y > 30 {
                println!("break inner loop if y > 30");
                break;
            }
        }
    };
    println!("Value of x y after loop: {}, {}", x, y);

    let mut i = 0;
    while i < 30 {
        i = i + 1;
    }
    println!("result of i after while loop: {}", i);

    let arr = [3, 5, 8, 13, 21];
    println!("Iterate through array elements.");
    for e in arr {
        println!("element: {}", e);
    }

    for count in (1..10).rev() {
        println!("{}", count);
    }
    println!("Liftoff!!");
}

fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moved from s1 to s2
    // This would cause error: [E0382] borrow of moved value: `s1`.
    // println!("value of s1: {}", s1);
    println!("value of s2: {}", s2);
    let s3 = s2.clone();
    println!("Value of s2: {}, and cloned version s3: {}", s2, s3);
    let x = 3;
    let y = x;
    println!("Primitive values are copied to stack so both x {} and y {} are valid", x, y);

    fn is_copy(i: i32) {
        println!("value of i: {}", i);
    }

    fn takes_ownership(s: String) {
        println!("value of s: {}", s);
    }

    fn takes_and_giveback(s: String) -> String {
        println!("value of s: {}", s);
        s
    }

    let txt = String::from("hello");
    let i1 = 5;

    takes_ownership(txt);
    is_copy(i1);

    println!("value of i1 after function call: {}", i1);
    // NOTE: This cause runtime error 'borrow of moved value'
    // println!("value of txt after function call: {}", txt);

    let txt = String::from("hello");
    let txt = takes_and_giveback(txt);
    println!("value of txt after takes and give back ownership: {}", txt);


}

fn reference() {
    fn takes_reference(s: &String) -> usize {
        s.len()
    }

    let txt = String::from("hello");
    let len = takes_reference(&txt);
    println!("length of {}: {}", txt, len);

    let mut txt = String::from("hello");

    // following code won't work
    // fn change_wont_work(s: &String) {
    //     s.push_str(", world");
    // }
    // change_wont_work(&txt);
    // println!("value after change: {}", txt);

    fn change_works(s: &mut String) {
        s.push_str(", world");
    }

    change_works(&mut txt);
    println!("value after change: {}", txt);

    {
        // NOTE: This works fine because ownership of pt1
        // ends after block scope
        let pt1 = &mut txt;
        println!("value of pt2: {}", pt1);
    }

    // NOTE: This cases [E0499] cannot borrow `txt` as mutable more than once at a time. 
    // let pt1 = &mut txt;
    // let pt2 = &mut txt;
    // println!("value of pt1 & pt2: {} {}", pt1, pt2);

    // NOTE: This cause [E0502] cannot borrow `txt` as mutable because it is also borrowed as immutable. 
    // let pt1 = &txt;
    // let pt2 = &mut txt;
    // println!("value of pt1 & pt2: {} {}", pt1, pt2);

    let pt1 = &txt;
    let pt2 = &txt;
    println!("value of pt1 & pt2: {}, {}", pt1, pt2);
    let pt3 = &mut txt;
    // NOTE: This works fine because lifetime of pt1 & pt2 already ends here
    println!("value of pt3: {}", pt3);

    // NOTE: This won't work cuz scope of s ends after function call,
    // so returning a reference of dropped variable is invalid.
    // fn danble() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }

    // NOTE: This works fine because we're returning the ownership
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
    let txt = no_dangle();
    println!("value of txt created inside function: {}", txt);
}

fn slice_type() {
    fn first_word_pos(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        return s.len();
    }

    let mut txt = String::from("Hello world!");
    let fst_wrd_pos = first_word_pos(&txt);
    println!("position of first world: {}", fst_wrd_pos);
    println!("first world: {}", &txt[..fst_wrd_pos]);

    txt.clear();
    // NOTE: Though this compiles fine,
    println!("position of first world: {}", fst_wrd_pos);
    // Following line cause panic because txt has been emptied
    // println!("the first world: {}", &txt[..fst_wrd_pos]);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        return &s[..]
    }

    let mut txt = String::from("Hello world!");
    let fst_wrd = first_word(&txt);
    println!("first word: {}", &fst_wrd);

    txt.clear();
    // NOTE: Accessing slice cause [E0502] cannot borrow `txt` as mutable because it is also borrowed as immutable. 
    // println!("first word: {}", &fst_wrd);
}