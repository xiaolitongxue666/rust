use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    used_names: Rc<RefCell<Vec<String>>>,
}

pub struct Robot {
    name: String,
    used_names: Rc<RefCell<Vec<String>>>,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            used_names: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn generate_name<R: Rng>(rng: &mut R) -> String {
        let letter1 = rng.random_range(b'A'..=b'Z') as char;
        let letter2 = rng.random_range(b'A'..=b'Z') as char;
        let digit1 = rng.random_range(0..=9);
        let digit2 = rng.random_range(0..=9);
        let digit3 = rng.random_range(0..=9);
        format!("{}{}{}{}{}", letter1, letter2, digit1, digit2, digit3)
    }

    fn get_unique_name<R: Rng>(used_names: &Rc<RefCell<Vec<String>>>, rng: &mut R) -> String {
        let mut names = used_names.borrow_mut();
        loop {
            let name = Self::generate_name(rng);
            if !names.contains(&name) {
                names.push(name.clone());
                return name;
            }
        }
    }

    fn release_name(used_names: &Rc<RefCell<Vec<String>>>, name: &str) {
        let mut names = used_names.borrow_mut();
        if let Some(pos) = names.iter().position(|n| n == name) {
            names.remove(pos);
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let used_names = Rc::clone(&self.used_names);
        let name = Self::get_unique_name(&used_names, rng);
        Robot { name, used_names }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        RobotFactory::release_name(&self.used_names, &self.name);
        self.name = RobotFactory::get_unique_name(&self.used_names, rng);
    }
}
