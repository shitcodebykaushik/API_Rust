// This is simple backebd api for the user validation .




use std::io::stdin;
#[allow(unused_variables)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
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
        output.push_str("Your apassword is ok!\n");
    } else {
        output.push_str("Your password is wrong\n");
    }

    output // Return the output string
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}



// This is the example of the  dynamic path in the routing . http://localhost:8000/data/raja   way to acces the resource .
# [macro_use ] extern crate rocket;
# [get ("/data/<say>")]
fn welcome(say: &str) -> String {
    format!("Hello, {}!",say)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![welcome])
}
