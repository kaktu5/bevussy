#![allow(dead_code)]
use bevy::prelude::*;
use rand::{rng, Rng};
use std::{array::from_fn, collections::HashMap};

pub struct VoxelEnginePlugin;

impl Plugin for VoxelEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelEngine>()
            .add_systems(Startup, setup);
    }
}

const CHUNK_SIZE: usize = 64;

type VoxelData = Option<[[[u16; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>;

#[derive(Debug, Default, Resource)]
struct VoxelEngine {
    chunks: HashMap<IVec3, VoxelData>,
}

fn f() {
    let mut rng = rng();
    let chunk: [[[bool; 8]; 8]; 8] = from_fn(|_| from_fn(|_| from_fn(|_| rng.random_bool(0.5))));
    let x: [[u8; 8]; 8] = chunk
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .fold(0u8, |acc, (bit_position, &value)| {
                            acc | ((value as u8) << bit_position)
                        })
                })
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; 8]>>()
        .try_into()
        .unwrap();

    println!("{:#?}", x);
}

fn setup() {
    f();
}
