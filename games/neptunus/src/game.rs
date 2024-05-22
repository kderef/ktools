use raylib::{
    camera::Camera3D, color::Color, drawing::{RaylibDraw, RaylibDraw3D, RaylibDrawHandle, RaylibMode3DExt}, math::Matrix, models::RaylibModel, RaylibHandle, RaylibThread
};

use crate::{assets::Assets, context::GameContext, rl_misc::vec3};

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Menu { camera: Camera3D, rotation: f32 },
    Game,
    Paused,
}

pub struct Game<'a> {
    rl: RaylibHandle,
    thread: &'a RaylibThread,
    gamestate: GameState,
    assets: Assets,
    context: GameContext,
}

impl<'a> Game<'a> {
    pub fn new(rl: RaylibHandle, thread: &'a RaylibThread, assets: Assets) -> Self {
        Self {
            rl,
            thread,
            assets,
            gamestate: GameState::Menu {
                camera: Camera3D::perspective(
                    vec3(15.0, 1.0, 10.0),
                    vec3(0.0, 0.0, 12.0),
                    vec3(0.0, 1.0, 0.0),
                    45.0,
                ),
                rotation: 0.0,
            },
            context: GameContext::default(),
        }
    }
    pub fn running(&self) -> bool {
        !self.rl.window_should_close()
    }
    pub fn update(&mut self) {
        self.context.update(&self.rl);

        let mut dr: RaylibDrawHandle;

        match self.gamestate {
            GameState::Menu {
                camera,
                ref mut rotation,
            } => {
                // update
                const ROTATION_SPEED: f32 = 2.0;

                let new_rotation = *rotation + ROTATION_SPEED * self.context.dt;
                *rotation = if new_rotation > 360.0 {
                    0.0
                } else {
                    new_rotation
                };

                self.assets.models.neptune.set_transform(
                    &Matrix::rotate_y(rotation.to_radians())
                );

                // draw
                dr = self.rl.begin_drawing(self.thread);
                dr.clear_background(Color::BLACK);

                let mut scene = dr.begin_mode3D(camera);

                scene.draw_model(
                    &self.assets.models.neptune,
                    vec3(0.0, 0.0, 5.0),
                    10.0,
                    Color::WHITE,
                );
                drop(scene);
            }
            _ => {
                dr = self.rl.begin_drawing(self.thread);
            }
        }

        dr.draw_fps(0, 0);
    }
}
