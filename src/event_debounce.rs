// use std::collections::HashMap;
// use std::hash::Hash;
//
// use std::time::{Duration, Instant};
//
// #[derive(Clone)]
// pub struct EventDebouncer<T>
// where
//     T: Eq + Hash + Clone + Send + 'static,
// {
//     delay: Duration,
//     last_call: HashMap<T, Instant>,
// }
//
// impl<T> EventDebouncer<T>
// where
//     T: Eq + Hash + Clone + Send + 'static,
// {
//     pub fn new(delay: Duration) -> Self {
//         EventDebouncer {
//             delay,
//             last_call: HashMap::new(),
//         }
//     }
//
//     pub fn ready(&mut self, message: &T) -> bool {
//         let now = Instant::now();
//
//         match self.last_call.get(message) {
//             Some(&last_time) if now.duration_since(last_time) < self.delay => false,
//             _ => {
//                 self.last_call.insert(message.clone(), now);
//                 true
//             }
//         }
//     }
// }
