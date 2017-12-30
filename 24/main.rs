extern crate read_input;

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
            used: pins == 0,
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

fn build_bridge(components: &Vec<Component>, zero_components: &Vec<Component>, current_component: &mut Component, used_components: HashSet<String>, layer: usize) -> Vec<usize> {
    let mut found = false;
    let mut lengths = Vec::new();
    for component in components {
        if used_components.contains(&component.id) {
            continue
        }

        let mut copy = component.clone();
        let mut current_clone = current_component.clone();
        if current_clone.can_match(&mut copy) {
            let mut used_components = used_components.clone();
            used_components.insert(copy.id.clone());
            found = true;
            lengths.append(&mut build_bridge(components, zero_components, &mut copy, used_components, layer + 1));
        }
    }

    if !found {
        let mut sum = zero_components.iter().filter(|component| {
            used_components.contains(&component.id)
        }).fold(0, |sum, component| sum + component.strength);

        sum += components.iter().filter(|component| {
            used_components.contains(&component.id)
        }).fold(0, |sum, component| sum + component.strength);

        lengths.push(sum);
    }

    lengths
}

fn main() {
    let text = match read_input::read_text("24/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut zero_components = Vec::new();
    let mut non_zero_components = Vec::new();

    for line in text.lines() {
        let mut iter = line.split("/");
        let left: usize = iter.next().unwrap().parse().unwrap();
        let right: usize = iter.next().unwrap().parse().unwrap();
        let component = Component::new(Port::new(left), Port::new(right), left + right);
        if component.is_zero() {
            zero_components.push(component);
        } else {
            non_zero_components.push(component);
        }
    }

    let mut lengths = Vec::new();
    for zero_component in &zero_components {
        let mut used_components = HashSet::new();
        used_components.insert(zero_component.id.clone());

        lengths.append(
            &mut build_bridge(
                &non_zero_components,
                &zero_components,
                &mut zero_component.clone(),
                used_components,
                1
            )
        );
    }

    lengths.sort();
    println!("{}", lengths[lengths.len() - 1]);
}
