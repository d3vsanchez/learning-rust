trait RouteStrategy {
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>; 
}

struct WalkStrategy{}
impl RouteStrategy for WalkStrategy{
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>{
        let speed_in_meters_by_seconds: u64 = 1; //meters per second
        Ok(format!("It will take {} seconds.", distance_in_meters/speed_in_meters_by_seconds))
    }
}

struct CarStrategy{}
impl RouteStrategy for CarStrategy{
    fn calculate_route(&self, distance_in_meters: u64) -> Result<String, String>{
        let speed_in_meters_by_seconds: u64 = 12; //meters per second
        Ok(format!("It will take {} seconds.", distance_in_meters/speed_in_meters_by_seconds))
    }
}

struct Navigator {
    route_stratey: Box<dyn RouteStrategy>
}

impl Navigator

