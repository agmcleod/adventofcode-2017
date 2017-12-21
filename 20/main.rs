extern crate read_input;
extern crate regex;

use regex::Regex;
use std::cmp;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Particle {
    pos: [i64; 3],
    vel: [i64; 3],
    accel: [i64; 3],
    distance_from_center: i64,
}

fn part_one(mut particles: Vec<Particle>) {
    let mut highest = 0;
    for _ in 0..1000 {
        for particle in particles.iter_mut() {
            particle.vel[0] += particle.accel[0];
            particle.vel[1] += particle.accel[1];
            particle.vel[2] += particle.accel[2];

            particle.pos[0] += particle.vel[0];
            particle.pos[1] += particle.vel[1];
            particle.pos[2] += particle.vel[2];

            let distance = particle.pos[0].abs() + particle.pos[1].abs() + particle.pos[2].abs();
            particle.distance_from_center = distance;
            highest = cmp::max(highest, distance);
        }
    }

    let mut lowest_distance = highest;
    let mut lowest_particle = 0;

    for (i, particle) in particles.iter().enumerate() {
        if lowest_distance > particle.distance_from_center {
            lowest_distance = particle.distance_from_center;
            lowest_particle = i;
        }
    }
    println!("{}", lowest_particle);
}

fn part_two(mut particles: Vec<Particle>) {
    for _ in 0..5000 {
        let mut coords_crossed = HashMap::new();
        for particle in particles.iter_mut() {
            particle.vel[0] += particle.accel[0];
            particle.vel[1] += particle.accel[1];
            particle.vel[2] += particle.accel[2];

            particle.pos[0] += particle.vel[0];
            particle.pos[1] += particle.vel[1];
            particle.pos[2] += particle.vel[2];

            let pos = (particle.pos[0], particle.pos[1], particle.pos[2]);
            if coords_crossed.contains_key(&pos) {
                let value = *coords_crossed.get(&pos).unwrap();
                coords_crossed.insert(pos, value + 1);
            } else {
                coords_crossed.insert(pos, 1);
            }
        }

        let mut new_particles = Vec::new();
        for particle in &particles {
            let pos = (particle.pos[0], particle.pos[1], particle.pos[2]);
            if let Some(count) = coords_crossed.get(&pos) {
                if *count == 1 {
                    new_particles.push(particle.clone());
                }
            } else {
                new_particles.push(particle.clone());
            }
        }
        particles = new_particles;
    }

    println!("{}", particles.len());
}

fn main() {
    let text = match read_input::read_text("20/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let re: Regex = Regex::new(r"p|a|v|=|<|>|\s").unwrap();

    let mut particles = Vec::new();

    for line in text.lines() {
        let mut pos: [i64; 3] = [0, 0, 0];
        let mut vel: [i64; 3] = [0, 0, 0];
        let mut accel: [i64; 3] = [0, 0, 0];
        for (i, n) in re.replace_all(line, "").split(",").enumerate() {
            if i < 3 {
                pos[i] = n.parse().unwrap();
            } else if i < 6 {
                vel[i % 3] = n.parse().unwrap();
            } else {
                accel[i % 3] = n.parse().unwrap();
            }
        }

        particles.push(Particle{ pos, vel, accel, distance_from_center: pos[0].abs() + pos[1].abs() + pos[2].abs() });
    }

    let particles_one = particles.clone();
    part_one(particles_one);
    part_two(particles);
}