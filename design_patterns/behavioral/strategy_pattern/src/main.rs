trait RouteStrategy {
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>; 
}

struct WalkStrategy{}
impl RouteStrategy for WalkStrategy{
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>{
        let speed_in_meters_by_seconds: u64 = 1; //meters per second
        Ok(format!("It will take {} seconds by walking", distance_in_meters/speed_in_meters_by_seconds))
    }
}

struct CarStrategy{}
impl RouteStrategy for CarStrategy{
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>{
        let speed_in_meters_by_seconds: u64 = 12; //meters per second
        Ok(format!("It will take {} seconds by driving", distance_in_meters/speed_in_meters_by_seconds))
    }
}

struct Navigator {
    route_strategy: Box<dyn RouteStrategy>
}
impl Navigator {
    fn new(route_strategy: Box<dyn RouteStrategy>) -> Self {
        Self { route_strategy }
    }
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>{
        self.route_strategy.calculate_route(distance_in_meters)
    }
}

fn main() {

    let distance_in_meters = 1500u64;

    let navigator = Navigator::new(Box::new(CarStrategy{}));
    match navigator.calculate_route(distance_in_meters) {
        Ok(sucess) => println!("{}", sucess),
        Err(error) => println!("{}", error),
    }

    let distance_in_meters = 1500u64;

    let navigator = Navigator::new(Box::new(WalkStrategy{}));
    match navigator.calculate_route(distance_in_meters) {
        Ok(sucess) => println!("{}", sucess),
        Err(error) => println!("{}", error),
    }
}
