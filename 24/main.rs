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

fn build_bridge(components: &Vec<Component>, current_component: &mut Component, used_components: HashSet<String>, bridge: Vec<Component>) -> Vec<Vec<Component>> {
    let mut found = false;
    let mut bridges = Vec::new();
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
            let mut bridge = bridge.clone();
            bridge.push(copy.clone());
            bridges.append(&mut build_bridge(components, &mut copy, used_components, bridge));
        }
    }

    if !found {
        bridges.push(bridge);
    }

    bridges
}

fn main() {
    let text = match read_input::read_text("24/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut zero_components = Vec::new();
    let mut all_components = Vec::new();

    for line in text.lines() {
        let mut iter = line.split("/");
        let left: usize = iter.next().unwrap().parse().unwrap();
        let right: usize = iter.next().unwrap().parse().unwrap();
        let component = Component::new(Port::new(left), Port::new(right), left + right);
        if component.is_zero() {
            zero_components.push(component.clone());
        }
        all_components.push(component);
    }

    let mut bridges = Vec::new();
    for zero_component in &zero_components {
        let mut used_components = HashSet::new();
        used_components.insert(zero_component.id.clone());

        let mut zero_component = zero_component.clone();
        // we know that the left port is the zero, so set it to used
        zero_component.left.used = true;
        let bridge = vec![zero_component.clone()];

        bridges.append(
            &mut build_bridge(
                &all_components,
                &mut zero_component,
                used_components,
                bridge,
            )
        );
    }

    let mut strongest_bridge = 0;
    let mut longest_bridge_length = 0;
    for bridge in &bridges {
        strongest_bridge = max(strongest_bridge, bridge.iter().fold(0, |sum, component| sum + component.strength));
        longest_bridge_length = max(longest_bridge_length, bridge.len());
    }

    let mut strongest_longest_bridge = 0;

    for bridge in &bridges {
        if bridge.len() == longest_bridge_length {
            strongest_longest_bridge = max(strongest_longest_bridge, bridge.iter().fold(0, |sum, component| sum + component.strength));
        }
    }

    println!("{}", strongest_bridge);
    println!("{}", strongest_longest_bridge);
}
