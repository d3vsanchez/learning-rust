trait Subscriber {
    fn notify(&self, message: &str);}

struct SubscriberA;
impl Subscriber for SubscriberA{
    fn notify(&self, message: &str) {
        println!("SubscriberA recieved the message: {}", message)}}

struct SubscriberB;
impl Subscriber for SubscriberB{
    fn notify(&self, message: &str) {
        println!("SubscriberB recieved the message: {}", message)}}

struct Publisher<'a> {
    subscribers: Vec<&'a dyn Subscriber>}
impl<'a> Publisher<'a> {
    fn new() -> Self {
        Self { subscribers: vec![]}}
    fn add_subscriber(&mut self, subscriber: &'a dyn Subscriber){
        self.subscribers.push(subscriber)}
    fn broadcast(&self, message: &str) {
        self.subscribers
            .iter()
            .for_each(|subscriber| {
                subscriber.notify(message)});}}

fn main() {
    let subscriber_a = SubscriberA{};
    let subscriber_b = SubscriberB{};
    
    let mut publisher = Publisher::new();

    publisher.add_subscriber(&subscriber_a);
    publisher.add_subscriber(&subscriber_b);

    publisher.broadcast(&"Hello Observable Pattern");
}