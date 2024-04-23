fn main() {
    /* The let mut shows the variable is muteable, aka you can manipulate the value directly. if a variable is declared with let keyword only, it is unmuteable and considered constant. */
    let mut counter = 0;
    /* In println!(), you have to pass variables in a literal {} tag.  */
    println!("{counter}");
    counter += 1;
    println!("{counter}");
    counter -= 2;
    println!("{counter}");
    /* Here we also declare two strings.
    There is a difference between them, however. 
    The type String is "owned" by you, the dev. It can be declared mut or just kept a constant. It's up to you.
    The type &str is a borrowed string slice, and you usually cannot manipulate it in any way.
     */
    let s1 = String::from("Hello");
    println!("{s1}");
    let s2: &str = "Hello everyone!";
    println!("{s2}");
    /* Some string methods */

    /* Appending.
    You can append to a string:
     */
    /* Here i use the method String.push_str() to append another string to my original string. First we clone our immuteable s1 into a muteable clone.*/
    let  mut s1_clone = s1.clone();
    s1_clone.push_str(", world!");
    println!("{s1_clone}");
    /* Concatenation.
    This is technically mutating s1 given its position in the operation.
    Since we declare a hidden variable that is a clone of s1, and mutate that. This allows s1 to stay constant, but we still get the effect of concatenating it.
    .clone() can hamper performance however, since it declares a new variable every time. 
    Concatenation using the + operator requires the first string to be muteable, hence using a hidden variable s1.clone().
     */
    /* Notice how, even though we don't have ownership over s2, we can still use it here. since we don't mutate the s2 variable. */
    let s3 = s1.clone() + ", world! " + s2;
    println!("{s3}");

    /* Here we use the format! macro to insert some variables into a template literal. We don't manipulate any of the inputs,
    so there is no need to clone or redeclare them.*/
    let s4 = format!("{}{} {}", s1,", World!", s2);
    println!("{s4}");
    /* Here we use the replace method, to replace a section matching out from: input to our to: input.
    We don't have to clone s1 here. since replace doesn't try to mutate the original value, but instead returns a new string. */
    let s5 = s1.replace("Hello", "Hi");
    println!("{s5}");
    /* Here we slice out a part of s2 into its own variable s5. We can do this to s2, since the s2 variable is still
    untouched. */
    let s5 = &s2[6..14];
    println!("{s5}");


    /* Practise */
    let mut practise_string: String = String::from("Learning Rust is fun!");
    println!("{practise_string}");
    practise_string.push_str(" And Challenging!");
    println!("{practise_string}");
    /* Notice here, the replace method returns a new string with the replaced part. */
    let new_practise_string = practise_string.replace("Challenging!", "Rewarding!");
    println!("{new_practise_string}");
    let number = 6;
    loop {
        if number % 4 == 0 {
            println!("{} is divisible by Four!", number);
            break;
        } if number % 3 == 0 {
            println!("{} is divisible by Three!", number);
            break;
        } if number % 2 == 0 {
            println!("{} is divisible by Two!", number);
            break
        }
    }
    for i in 1..5 {
        println!("I have looped {} times so far!", i)
    }
    let mut count_start = 5;
    while count_start != 0 {
        println!("{}...", count_start);
        count_start -= 1;
    }
    println!("LIFT OFF!")
}


