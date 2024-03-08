trait ServiceInterface {
    fn operation(&mut self) -> Result<(), ()>;}

struct Service;
impl ServiceInterface for Service {
    fn operation(&mut self) -> Result<(), ()> {
        println!("Calling from the real service");
        Ok(())}}

struct Proxy {
    service: Option<Service>,}

impl Proxy {
    fn new() -> Self {
        Proxy { service: None }}
    fn initialize_service_lazily(&mut self) {
        self.service = Some(Service {});}}

impl ServiceInterface for Proxy {
    fn operation(&mut self) -> Result<(), ()> {
        match &mut self.service {
            Some(service) => service.operation(),
            None => {
                println!("Lazy initialization of service");
                self.initialize_service_lazily();
                self.operation()}}}}

fn main() {
    let mut proxy = Proxy::new();

    match proxy.operation() {
        Ok(_) => println!("Proxy executed successfully"),
        Err(_) => println!("Error while executing proxy"),}}