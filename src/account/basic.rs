use actix_web::HttpRequest;


pub fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

pub fn register(req: HttpRequest) -> &'static str{
    println!("welcome to register");
    "welcome to register"
}
