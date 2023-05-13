use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use nannou::draw::background::new;
use nannou::prelude::*;
extern crate nalgebra as na;
use na::{Vector2};
use std::f32::consts::{PI};
use std::env;

// [[1,0], [0,1], [-1,0], [0,-1]]
// [[1,1], [-1,1], [-1,-1], [1,-1]]


fn move_sand_better(sandbox: &mut Vec<Vec<i32>>, maxed_piles: &mut HashSet<Vector2<i32>>, pattern: &Vec<Vector2<i32>>) {    
    if let Some(current_pile) = maxed_piles.iter().next().cloned() {
        let value = &mut sandbox[current_pile.x as usize][current_pile.y as usize];
        *value -= pattern.len() as i32;

        if *value < pattern.len() as i32 {
            maxed_piles.remove(&current_pile);
        }

        for sand_move in pattern {
            let move_x = current_pile.x + sand_move.x;
            let move_y = current_pile.y + sand_move.y;

            let move_location = &mut sandbox[move_x as usize][move_y as usize];
            *move_location += 1;

            if *move_location >= pattern.len() as i32 {
                maxed_piles.insert(Vector2::new(move_x, move_y));
            }
        }
    }
}


struct Model {
    _window: window::Id,
    sandbox: Vec<Vec<i32>>,
    pattern: Vec<Vector2<i32>>,
    canvas_size: usize,
    maxed_piles: HashSet<Vector2<i32>>
}

fn model(app: &App) -> Model {
    let canvas_size = 4000;

    let _window = app.new_window().view(view).build().unwrap();
    let mut sandbox: Vec<Vec<i32>> = vec![vec![0; canvas_size]; canvas_size];
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

    let center = canvas_size/2;

    let mut maxed_piles: HashSet<Vector2<i32>> = HashSet::new();

    Model {_window, sandbox, pattern, canvas_size, maxed_piles}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for i in 0..1000000 {
        move_sand_better(&mut _model.sandbox, &mut _model.maxed_piles, &_model.pattern);

        if _model.maxed_piles.len() == 0 {
            _model.maxed_piles.insert(Vector2::new((_model.canvas_size/2) as i32, (_model.canvas_size/2) as i32));
            _model.sandbox[_model.canvas_size/2][_model.canvas_size/2] = _model.pattern.len() as i32;
        }
    }
    println!("{}", _model.maxed_piles.len());
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let scalar = 1.0;
    draw.background().color(BLACK);

    for i in 0.._model.canvas_size {
        for j in 0.._model.canvas_size {
            let value =  _model.sandbox[i][j];
            if value != 0 {
                let color = match _model.sandbox[i][j] {
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
                    _ => WHITE,
                };

                let center = (_model.canvas_size/2) as f32;
                let x = (i as f32 - center) * scalar;
                let y = (j as f32 - center) * scalar;


                draw.rect().w(scalar).h(scalar).x_y(x, y).color(color);
            }
        };
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {

    nannou::app(model).loop_mode(LoopMode::rate_fps(30.0)).update(update).run();
}

