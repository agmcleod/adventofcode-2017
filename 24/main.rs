extern crate read_input;

use std::cmp::Ordering;
use std::cmp::max;
use std::fmt;
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Port {
    pins: usize,
    used: bool,
}

impl Port {
    fn new(pins: usize) -> Port {
        Port{
            pins,
            used: false,
        }
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Port) -> bool {
        self.pins == other.pins && !self.used && !other.used
    }
}

struct Component {
    left: Port,
    right: Port,
    strength: usize,
    id: String,
}

impl Component {
    fn new(left: Port, right: Port, strength: usize) -> Component {
        Component{
            left,
            right,
            strength,
            id: format!("{}/{}", left.pins, right.pins),
        }
    }

    fn can_match(&mut self, other: &mut Component) -> bool {
        if self.left == other.left {
            self.left.used = true;
            other.left.used = true;
            return true
        } else if self.left == other.right {
            self.left.used = true;
            other.right.used = true;
            return true
        } else if self.right == other.right {
            self.right.used = true;
            other.right.used = true;
            return true
        } else if self.right == other.left {
            self.right.used = true;
            other.left.used = true;
            return true
        }

        false
    }

    fn is_zero(&self) -> bool {
        self.left.pins == 0 || self.right.pins == 0
    }
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{} ({})", self.left.pins, self.right.pins, self.strength)
    }
}

impl Clone for Component {
    fn clone(&self) -> Component {
        Component{
            left: self.left,
            right: self.right,
            strength: self.strength,
            id: self.id.clone(),
        }
    }
}

fn main() {
    let text = match read_input::read_text("24/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut components = Vec::new();

    for line in text.lines() {
        let mut iter = line.split("/");
        let left: usize = iter.next().unwrap().parse().unwrap();
        let right: usize = iter.next().unwrap().parse().unwrap();
        components.push(Component::new(Port::new(left), Port::new(right), left + right));
    }

    components.sort_by(|a, b| {
        if a.is_zero() && b.is_zero() {
            Ordering::Equal
        } else if a.is_zero() && !b.is_zero() {
            Ordering::Less
        } else if !a.is_zero() && b.is_zero() {
            Ordering::Greater
        } else {
            b.strength.cmp(&a.strength)
        }
    });

    let mut try_zero_index = 0;
    let mut max_zero_component_index = 0;

    for component in &components {
        if component.is_zero() {
            max_zero_component_index += 1;
        } else {
            break
        }
    }

    let mut strength_total = 0;
    let mut start_index = 0;

    loop {
        let mut bridge = vec![components.get(try_zero_index).unwrap().clone()];

        let mut used_set = HashSet::new();
        loop {
            let mut iter = components.iter().skip(max_zero_component_index + start_index);
            let mut found = false;
            while let Some(component) = iter.next() {
                if used_set.contains(&component.id) {
                    continue
                }
                used_set.insert(component.id.clone());
                let len = bridge.len();
                let mut component_clone = component.clone();
                let did_match = {
                    let last = bridge.get_mut(len - 1).unwrap();
                    last.can_match(&mut component_clone)
                };

                if did_match {
                    bridge.push(component_clone);
                    found = true;
                }
            }

            if !found {
                break
            }
        }


        strength_total = max(strength_total, bridge.iter().fold(0, |sum, component| sum + component.strength));

        try_zero_index += 1;
        if try_zero_index >= max_zero_component_index {
            if start_index + try_zero_index == components.len() - 1 {
                break
            }
            start_index += 1;
            try_zero_index = 0;
        }
    }

    println!("strength: {}", strength_total);
}
