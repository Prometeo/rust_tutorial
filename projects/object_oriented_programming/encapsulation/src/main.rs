// we can use the pub keyword to decide which modules, types, functions,
// and methods in our code should be public, and by default everything
// else is private.

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Inheritance as a Type System and as Code Sharing

// Inheritance is a mechanism whereby an object can inherit from another object’s definition,
// thus gaining the parent object’s data and behavior without you having to define them again.

// You choose inheritance for two main reasons. One is for reuse of code:
// You can share Rust code using default trait method implementations instead
// This is similar to a parent class having an implementation of a method and an inheriting
// child class also having the implementation of the method.

// The other reason to use inheritance relates to the type system: to enable a child type
// to be used in the same places as the parent type. This is also called polymorphism
