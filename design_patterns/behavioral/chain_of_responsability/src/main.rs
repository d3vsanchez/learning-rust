trait Handler {
    fn handle(&self, request: i32);
}

struct FizzBuzzHandler {
    next: Option<Box<dyn Handler>>,
}
impl Handler for FizzBuzzHandler {
    fn handle(&self, request: i32) {
        if request % 15 == 0 {
            println!("FizzBuzz")
        }
        match &self.next {
            Some(next) => next.handle(request),
            None => (),
        };
    }
}

struct FizzHandler {
    next: Option<Box<dyn Handler>>,
}
impl Handler for FizzHandler {
    fn handle(&self, request: i32) {
        if request % 5 == 0{
            println!("Fizz")
        }
        match &self.next {
            Some(next) => next.handle(request),
            None => (),
        };
    }
}

struct BuzzHandler {
    next: Option<Box<dyn Handler>>,
}
impl Handler for BuzzHandler {
    fn handle(&self, request: i32) {
        if request % 3 == 0{
            println!("Buzz")
        }
        match &self.next {
            Some(next) => next.handle(request),
            None => (),
        };
    }
}

struct FallbackHandler {}
impl Handler for FallbackHandler {
    fn handle(&self, request: i32) {
        println!("{}", request);
    }
}

fn main() {
    let default_handler = FallbackHandler{};
    let buzz_handler = BuzzHandler{ next: Some(Box::new(default_handler)) };
    let fizz_handler = FizzHandler{ next: Some(Box::new(buzz_handler)) };
    let configured_chain = FizzBuzzHandler{ next: Some(Box::new(fizz_handler)) };

    for n in 1..50 {
        configured_chain.handle(n)
    }
}