use std::collections::HashMap;
use nannou::draw::background::new;
use nannou::prelude::*;
extern crate nalgebra as na;
use na::{Vector2};
use std::f32::consts::{PI};
use std::env;

fn find_max_piles(sandbox: &HashMap<Vector2<i32>, i32>, pattern_size: usize) -> Vec<Vector2<i32>> {

    let mut maxed_piles: Vec<Vector2<i32>> = Vec::new();

    for (pos, pile) in sandbox {
        if *pile >= pattern_size as i32 {
            maxed_piles.push(*pos);
        }
    }

    maxed_piles
}

// [[1,0], [0,1], [-1,0], [0,-1]]
// [[1,1], [-1,1], [-1,-1], [1,-1]]

fn move_sand(sandbox: &mut HashMap<Vector2<i32>, i32>, maxed_piles: &Vec<Vector2<i32>>, pattern: &Vec<Vector2<i32>>) {
    for pile_index in maxed_piles {
        let pile = sandbox.get_mut(&pile_index).unwrap();
        *pile -= pattern.len() as i32;

        for sand_move in pattern {
            let new_pos = pile_index + sand_move;

            let to_pile = sandbox.entry(new_pos).or_insert(0);
            *to_pile += 1;
        }
    }
}

struct Model {
    _window: window::Id,
    sandbox: HashMap<Vector2<i32>, i32>,
    pattern: Vec<Vector2<i32>>,
    start_points: Vec<Vector2<i32>>,
    sand_count: i64,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut sandbox: HashMap<Vector2<i32>, i32> = HashMap::new();
    let mut pattern: Vec<Vector2<i32>> = Vec::new();
    pattern.push(Vector2::new(0, 1));
    pattern.push(Vector2::new(1, 0));
    pattern.push(Vector2::new(0, -1));
    pattern.push(Vector2::new(-1, 0));


    // pattern.push(Vector2::new(0, 2));
    // pattern.push(Vector2::new(2, 0));
    // pattern.push(Vector2::new(0, -2));
    // pattern.push(Vector2::new(-2, 0));
    // pattern.push(Vector2::new(1, 1));
    // pattern.push(Vector2::new(1, -1));
    // pattern.push(Vector2::new(-1, 1));
    // pattern.push(Vector2::new(-1, -1));

    let mut start_points: Vec<Vector2<i32>> = Vec::new();
    start_points.push(Vector2::new(0,0));
    // start_points.push(Vector2::new(0,10));

    let sand_count: i64 = 0;

    Model {_window, sandbox, pattern, start_points, sand_count}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

    for i in 0..10 {
        for point in _model.start_points.iter() {
            let start =_model.sandbox.entry(*point).or_insert(262144);
            // *start += 1;
            // _model.sand_count += 1;

        } 
        
        // loop {
            let maxed_sand_piles = find_max_piles(&_model.sandbox, _model.pattern.len());
            // if maxed_sand_piles.len() == 0 {
                // break;
            // }

            move_sand(&mut _model.sandbox, &maxed_sand_piles, &_model.pattern);
        // }
    }

    // println!("Sand Count: {}", _model.sand_count)
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let scalar = 4.0;
    draw.background().color(BLACK);

    for (point, value) in _model.sandbox.iter() {
        let color = match value {
            1 => CORNFLOWERBLUE,
            2 => PURPLE,
            3 => CRIMSON,
            4 => GREEN,
            5 => YELLOW,
            6 => ORANGE,
            7 => DARKSEAGREEN,
            8 => CHOCOLATE,
            9 => DARKORCHID,
            10 => DEEPPINK,
            11 => CORNFLOWERBLUE,
            _ => BLACK,
        };

        draw.rect().w(scalar).h(scalar).x_y(scalar * point.x as f32, scalar * point.y as f32).color(color);
        // draw.rect().radius(scalar * 0.5).x_y(scalar * point.x as f32, scalar * point.y as f32).color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {

    nannou::app(model).loop_mode(LoopMode::rate_fps(30.0)).update(update).run();
}

