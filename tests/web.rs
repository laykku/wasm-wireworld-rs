//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

extern crate wasm_wireworld_rs;
use wasm_wireworld_rs::{Cell, World};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_new_world_created_empty() {
    let mut world = World::new(2, 2);

    let empty_cells = [
        (0, 0, Cell::Empty),
        (0, 1, Cell::ElectronHead),
        (1, 0, Cell::Empty),
        (1, 1, Cell::Empty),
    ];

    world.set_cells(&empty_cells);
}

// test wireworld rules

#[wasm_bindgen_test]
fn test_empty_stay_empty() {
    let mut world = World::new(2, 2);
    world.tick();

    let expected = &[Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];

    assert_eq!(world.get_cells(), expected);
}

#[wasm_bindgen_test]
fn test_empty_electron_head_become_tail() {
    let mut world = World::new(2, 2);
    world.set_cells(&[(0, 1, Cell::ElectronHead)]);
    world.tick();

    let expected = &[Cell::Empty, Cell::ElectronTail, Cell::Empty, Cell::Empty];

    assert_eq!(world.get_cells(), expected);
}

#[wasm_bindgen_test]
fn test_empty_electron_tail_become_conductor() {
    let mut world = World::new(2, 2);
    world.set_cells(&[(0, 1, Cell::ElectronTail)]);
    world.tick();

    let expected = &[Cell::Empty, Cell::Conductor, Cell::Empty, Cell::Empty];

    assert_eq!(world.get_cells(), expected);
}

#[wasm_bindgen_test]
fn test_conductor_become_electron_head_with_1_neighbour() {
    let mut world = World::new(3, 3);
    world.set_cells(&[(1, 1, Cell::Conductor), (0, 0, Cell::ElectronHead)]);
    world.tick();

    let expected = &[
        Cell::ElectronTail, // 0 0
        Cell::Empty,        // 0 1
        Cell::Empty,        // 0 2
        Cell::Empty,        // 1 0
        Cell::ElectronHead, // 1 1
        Cell::Empty,        // 1 2
        Cell::Empty,        // 2 0
        Cell::Empty,        // 2 1
        Cell::Empty,        // 2 2
    ];

    assert_eq!(world.get_cells(), expected);
}

#[wasm_bindgen_test]
fn test_conductor_become_electron_head_with_2_neighbours() {
    let mut world = World::new(3, 3);
    world.set_cells(&[
        (1, 1, Cell::Conductor),
        (0, 0, Cell::ElectronHead),
        (0, 1, Cell::ElectronHead),
    ]);
    world.tick();

    let expected = &[
        Cell::ElectronTail, // 0 0
        Cell::ElectronTail, // 0 1
        Cell::Empty,        // 0 2
        Cell::Empty,        // 1 0
        Cell::ElectronHead, // 1 1
        Cell::Empty,        // 1 2
        Cell::Empty,        // 2 0
        Cell::Empty,        // 2 1
        Cell::Empty,        // 2 2
    ];

    assert_eq!(world.get_cells(), expected);
}

#[wasm_bindgen_test]
fn test_conductor_not_become_electron_head_with_more_than_2_neighbours() {
    let mut world = World::new(3, 3);
    world.set_cells(&[
        (1, 1, Cell::Conductor),
        (0, 0, Cell::ElectronHead),
        (0, 1, Cell::ElectronHead),
        (0, 2, Cell::ElectronHead),
    ]);
    world.tick();

    let expected = &[
        Cell::ElectronTail, // 0 0
        Cell::ElectronTail, // 0 1
        Cell::ElectronTail, // 0 2
        Cell::Empty,        // 1 0
        Cell::Conductor,    // 1 1
        Cell::Empty,        // 1 2
        Cell::Empty,        // 2 0
        Cell::Empty,        // 2 1
        Cell::Empty,        // 2 2
    ];

    assert_eq!(world.get_cells(), expected);
}

#[cfg(test)]
fn clock() -> World {
    let mut world = World::new(64, 64);
    world.set_cells(&[
        (20, 20, Cell::Conductor),
        (20, 21, Cell::Conductor),
        (19, 22, Cell::Conductor),
        (21, 22, Cell::Conductor),
        (19, 21, Cell::Conductor),
        (21, 21, Cell::Conductor),
        (20, 23, Cell::Conductor),
        (20, 24, Cell::Conductor),
        (20, 25, Cell::Conductor),
        (21, 25, Cell::Conductor),
        (22, 25, Cell::Conductor),
        (23, 25, Cell::Conductor),
        (24, 25, Cell::Conductor),
        (24, 24, Cell::Conductor),
        (24, 23, Cell::Conductor),
        (23, 22, Cell::Conductor),
        (25, 22, Cell::Conductor),
        (23, 21, Cell::Conductor),
        (25, 21, Cell::Conductor),
        (24, 22, Cell::Conductor),
        (24, 20, Cell::Conductor),
        (24, 19, Cell::Conductor),
        (24, 18, Cell::Conductor),
        (23, 18, Cell::Conductor),
        (22, 18, Cell::ElectronHead),
        (21, 18, Cell::Conductor),
        (20, 18, Cell::Conductor),
        (20, 19, Cell::Conductor),
    ]);
    world
}

#[wasm_bindgen_test]
fn test_clock_9th_gen() {
    let mut clock = clock();

    clock.tick(); // 2nd gen
    clock.tick(); // 3rd gen
    clock.tick(); // 4th gen
    clock.tick(); // 5th gen
    clock.tick(); // 6th gen
    clock.tick(); // 7th gen
    clock.tick(); // 8th gen
    clock.tick(); // 9th gen

    let mut expected = World::new(64, 64);
    expected.set_cells(&[
        (20, 20, Cell::Conductor),
        (20, 21, Cell::Conductor),
        (19, 22, Cell::Conductor),
        (21, 22, Cell::Conductor),
        (19, 21, Cell::Conductor),
        (21, 21, Cell::Conductor),
        (20, 23, Cell::Conductor),
        (20, 24, Cell::ElectronTail),
        (20, 25, Cell::ElectronHead),
        (21, 25, Cell::ElectronHead),
        (22, 25, Cell::Conductor),
        (23, 25, Cell::Conductor),
        (24, 25, Cell::Conductor),
        (24, 24, Cell::Conductor),
        (24, 23, Cell::Conductor),
        (23, 22, Cell::Conductor),
        (25, 22, Cell::Conductor),
        (23, 21, Cell::Conductor),
        (25, 21, Cell::Conductor),
        (24, 22, Cell::Conductor),
        (24, 20, Cell::Conductor),
        (24, 19, Cell::Conductor),
        (24, 18, Cell::Conductor),
        (23, 18, Cell::Conductor),
        (22, 18, Cell::Conductor),
        (21, 18, Cell::Conductor),
        (20, 18, Cell::Conductor),
        (20, 19, Cell::Conductor),
    ]);

    assert_eq!(&clock.get_cells(), &expected.get_cells());
}

#[wasm_bindgen_test]
fn test_clock_10th_gen() {
    let mut clock = clock();

    clock.tick(); // 2nd gen
    clock.tick(); // 3rd gen
    clock.tick(); // 4th gen
    clock.tick(); // 5th gen
    clock.tick(); // 6th gen
    clock.tick(); // 7th gen
    clock.tick(); // 8th gen
    clock.tick(); // 9th gen
    clock.tick(); // 10th gen

    let mut expected = World::new(64, 64);
    expected.set_cells(&[
        (20, 20, Cell::Conductor),
        (20, 21, Cell::Conductor),
        (19, 22, Cell::Conductor),
        (21, 22, Cell::Conductor),
        (19, 21, Cell::Conductor),
        (21, 21, Cell::Conductor),
        (20, 23, Cell::Conductor),
        (20, 24, Cell::Conductor),
        (20, 25, Cell::ElectronTail),
        (21, 25, Cell::ElectronTail),
        (22, 25, Cell::ElectronHead),
        (23, 25, Cell::Conductor),
        (24, 25, Cell::Conductor),
        (24, 24, Cell::Conductor),
        (24, 23, Cell::Conductor),
        (23, 22, Cell::Conductor),
        (25, 22, Cell::Conductor),
        (23, 21, Cell::Conductor),
        (25, 21, Cell::Conductor),
        (24, 22, Cell::Conductor),
        (24, 20, Cell::Conductor),
        (24, 19, Cell::Conductor),
        (24, 18, Cell::Conductor),
        (23, 18, Cell::Conductor),
        (22, 18, Cell::Conductor),
        (21, 18, Cell::Conductor),
        (20, 18, Cell::Conductor),
        (20, 19, Cell::Conductor),
    ]);

    assert_eq!(&clock.get_cells(), &expected.get_cells());
}

#[wasm_bindgen_test]
fn test_clock_11th_gen() {
    let mut clock = clock();

    clock.tick(); // 2nd gen
    clock.tick(); // 3rd gen
    clock.tick(); // 4th gen
    clock.tick(); // 5th gen
    clock.tick(); // 6th gen
    clock.tick(); // 7th gen
    clock.tick(); // 8th gen
    clock.tick(); // 9th gen
    clock.tick(); // 10th gen
    clock.tick(); // 11th gen

    let mut expected = World::new(64, 64);
    expected.set_cells(&[
        (20, 20, Cell::Conductor),
        (20, 21, Cell::Conductor),
        (19, 22, Cell::Conductor),
        (21, 22, Cell::Conductor),
        (19, 21, Cell::Conductor),
        (21, 21, Cell::Conductor),
        (20, 23, Cell::Conductor),
        (20, 24, Cell::Conductor),
        (20, 25, Cell::Conductor),
        (21, 25, Cell::Conductor),
        (22, 25, Cell::ElectronTail),
        (23, 25, Cell::ElectronHead),
        (24, 25, Cell::Conductor),
        (24, 24, Cell::Conductor),
        (24, 23, Cell::Conductor),
        (23, 22, Cell::Conductor),
        (25, 22, Cell::Conductor),
        (23, 21, Cell::Conductor),
        (25, 21, Cell::Conductor),
        (24, 22, Cell::Conductor),
        (24, 20, Cell::Conductor),
        (24, 19, Cell::Conductor),
        (24, 18, Cell::Conductor),
        (23, 18, Cell::Conductor),
        (22, 18, Cell::Conductor),
        (21, 18, Cell::Conductor),
        (20, 18, Cell::Conductor),
        (20, 19, Cell::Conductor),
    ]);

    assert_eq!(&clock.get_cells(), &expected.get_cells());
}
