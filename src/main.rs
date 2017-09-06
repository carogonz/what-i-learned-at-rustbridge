extern crate pencil; //telling compiler there's a crate called pencil
extern crate pick_one; 
extern crate motivations; 

use pencil::{Pencil, Request, Response, PencilResult}; //from the crate we want to use these import modules  

use motivations::MOTIVATIONS;

use std::collections::BTreeMap;

pub fn handler(_: &mut Request) -> PencilResult { //this function will receive a request. 
    println! ("Received Request");
   // Ok(Response::from("Hello World!")) //the Ok needs a response based on the Pencil Result code. It needs a response from this string called Hello World! 
   let result=pick_one::pick_one_str(&MOTIVATIONS).to_string();
   Ok(Response::from(result))
}

fn main() { //writing the server 
    let mut server = Pencil::new("./src");
     server.enable_static_file_handling(); 
     server.register_template("motivation.html");
     server.get("/", "hello", motivation); //when a client accesses the endpoint "Hello" you have to handle the access with a handler (hello function in line 5) 
     server.run("127.0.0.1:5000"); //special address that refers to my computer. 127.0.0.1 = 0.0.0.0. = local host. Every address has 65,000 ports. Anything above 1,024 is safe to unrestrictive. 
} 

pub fn motivation(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    let motivation = pick_one::pick_one_str(&MOTIVATIONS).to_string();
    let crab = pick_one::pick_one_str(&["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"]).to_string();
    context.insert("motivation".to_string(), motivation);
    context.insert("image".to_string(), crab);
    return request.app.render_template("motivation.html", &context);
}
