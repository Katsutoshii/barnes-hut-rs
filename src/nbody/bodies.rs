use serde::{Deserialize, Serialize};
use std::fs;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct BodyPosition3D {
    pub m: Vec<f32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body3D {
    pub m: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BodyPosition2D {
    pub m: Vec<f32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body2D {
    pub m: f32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

pub fn load_bodies_2d(filename: &str) -> Vec<Body2D> {
    let data = fs::read_to_string(filename).expect("Error reading from file.");
    let bodies: Vec<Body2D> = serde_json::from_str(&data).unwrap();

    return bodies;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_2d() {
        let filename: &str = "data/systems/test2d.json";
        let bodies: Vec<Body2D> = load_bodies_2d(filename);
        for body in bodies {
            println!("{:?}", body);
        }
    }
}
