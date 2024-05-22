use raylib::{
    camera::Camera3D,
    color::Color,
    drawing::{RaylibDraw, RaylibDraw3D, RaylibDrawHandle, RaylibMode3DExt},
    math::Matrix,
    models::RaylibModel,
    RaylibHandle, RaylibThread,
};

use crate::{assets::Assets, context::GameContext, rl_misc::vec3, settings::Settings};

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
    ctx: GameContext,
    settings: Settings,
}

impl<'a> Game<'a> {
    pub fn new(mut rl: RaylibHandle, thread: &'a RaylibThread, assets: Assets) -> Self {
        rl.set_exit_key(None);
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
            ctx: GameContext::default(),
            settings: Settings::default(),
        }
    }
    pub fn running(&self) -> bool {
        !self.rl.window_should_close()
    }
    fn titlescreen(&mut self) -> RaylibDrawHandle {
        // update
        const ROTATION_SPEED: f32 = 2.0;

        let GameState::Menu {
            camera,
            ref mut rotation,
        } = self.gamestate
        else {
            unreachable!()
        };

        let new_rotation = *rotation + ROTATION_SPEED * self.ctx.dt;
        *rotation = if new_rotation > 360.0 {
            0.0
        } else {
            new_rotation
        };

        self.assets
            .models
            .neptune
            .set_transform(&Matrix::rotate_y(rotation.to_radians()));

        // draw
        let mut dr = self.rl.begin_drawing(self.thread);
        dr.clear_background(Color::BLACK);

        let mut scene = dr.begin_mode3D(camera);

        scene.draw_model(
            &self.assets.models.neptune,
            vec3(0.0, 0.0, 5.0),
            10.0,
            Color::WHITE,
        );
        drop(scene);
        dr
    }
    pub fn update(&mut self) {
        self.ctx.update(&self.rl);

        let mut dr = match self.gamestate {
            GameState::Menu { .. } => self.titlescreen(),
            _ => self.rl.begin_drawing(self.thread),
        };

        dr.draw_fps(0, 0);
    }
}
