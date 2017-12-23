extern crate read_input;

use std::collections::HashMap;

fn flatten_grid(grid: &Vec<Vec<&str>>) -> String {
    grid.iter().map(|row|
        row.iter().map(|s| s.to_string()).collect::<String>()
    ).collect::<Vec<String>>().join("/")
}

fn string_to_grid<'a>(string: &'a String) -> Vec<Vec<&'a str>> {
    string.split("/").map(|section| {
        section.split("").filter(|v| *v != "").collect()
    }).collect()
}

fn flip_grid<'a>(grid: &mut Vec<Vec<&str>>) {
    for row in grid.iter_mut() {
        row.reverse();
    }
}

fn rotate_grid_by_90(grid: &mut Vec<Vec<&str>>) {
    let mut rotated = Vec::new();
    let n = grid.len();

    for i in 0..n {
        rotated.push(Vec::new());
        for j in 0..n {
            rotated[i].push(grid[n - j - 1][i]);
        }
    }

    *grid = rotated;
}

fn get_divisor(grid: &Vec<Vec<&str>>) -> usize {
    if grid.len() % 2 == 0 {
        2
    } else if grid.len() % 3 == 0 {
        3
    } else {
        panic!("Grid not divisable by 2 or 3 {:?}", grid);
    }
}

// is there a such thing as too deep of an array?
fn get_sub_grids<'a>(grid: &Vec<Vec<&'a str>>) -> Vec<Vec<Vec<Vec<&'a str>>>> {
    let divisor = get_divisor(grid);
    let mut sub_grids = Vec::new();

    for row in 0..(grid.len() / divisor) {
        sub_grids.push(Vec::new());
        for _ in 0..(grid.len() / divisor) {
            sub_grids[row].push(vec![vec!["."; divisor]; divisor]);
        }
    }

    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            sub_grids[r / divisor][c / divisor][r % divisor][c % divisor] = col;
        }
    }

    sub_grids
}

fn main(){
    let text = match read_input::read_text("21/input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut transforms = HashMap::new();
    for line in text.lines() {
        let mut iter = line.split(" => ");
        transforms.insert(iter.next().unwrap().to_string(), iter.next().unwrap().to_string());
    }

    let mut grid = Vec::new();
    grid.push(vec![".", "#", "."]);
    grid.push(vec![".", ".", "#"]);
    grid.push(vec!["#", "#", "#"]);

    for _ in 0..18 {
        let mut sub_grids = get_sub_grids(&grid);
        for row in sub_grids.iter_mut() {
            for sub_grid in row.iter_mut() {
                let mut count = 1;
                loop {
                    let grid_as_string = flatten_grid(sub_grid);
                    if transforms.contains_key(&grid_as_string) {
                        *sub_grid = string_to_grid(transforms.get(&grid_as_string).unwrap());
                        break
                    }

                    rotate_grid_by_90(sub_grid);
                    count += 1;
                    if count == 5 {
                        flip_grid(sub_grid);
                    }
                    if count > 8 {
                        panic!("Could not find transform for: {:?}", grid_as_string);
                    }
                }
            }
        }

        let divisor = get_divisor(&grid);
        let new_size = (grid.len() / divisor) * (divisor + 1);
        let mut new_grid = vec![vec!["."; new_size]; new_size];
        for (r, row) in sub_grids.iter().enumerate() {
            for (c, sub_grid) in row.iter().enumerate() {
                for (sg_r, row) in sub_grid.iter().enumerate() {
                    for (sg_c, value) in row.iter().enumerate() {
                        new_grid[(r * (divisor + 1)) + sg_r][(c * (divisor + 1)) + sg_c] = value;
                    }
                }
            }
        }

        grid = new_grid;
    }

    let mut pixels_turned_on = 0;
    for row in &grid {
        for v in row {
            if *v == "#" {
                pixels_turned_on += 1;
            }
        }
    }

    println!("{}", pixels_turned_on);
}