// This is simple backebd api for the user validation .
use std::io::stdin;
#[allow(unused_variables)]
#[macro_use] extern crate rocket;

#[get("/")]  // This is routed method 
fn index() -> String {              // These are the rounte handler 
    let mut output = String::new();

    output.push_str("# Welcome to the user validation form\n");

    let mut a = String::new();
    stdin().read_line(&mut a).expect("Please enter the number");
    let username2 = String::from("Kaushik Raj");
    let a = a.trim();
    
    if a == username2 {
        output.push_str("Give your password details\n");
    } else {
        output.push_str("Try to remember your username\n");
    }

    let userpasswd1 = String::from("Try");
    let mut b = String::new();
    stdin().read_line(&mut b).expect("Please enter the password");
    let b = b.trim();
    
    if b == userpasswd1 {
        output.push_str("Your password is ok!\n");
    } else {
        output.push_str("Your password is wrong\n");
    }

    output // Return the output string
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
