use crate::mino_types::*;
use bevy::{ecs::system::Commands, prelude::*};
use rand::Rng;

use crate::meta::*;

pub struct MinoPlugin;

impl Plugin for MinoPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_sprites)
            .add_startup_system(register_mino_patterns)
            .add_startup_system(startup)
            .add_system(position_transform)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_mino_one_loop);
    }
}

#[derive(Component, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct MinoPattern {
    mino_type: MinoTypes,
    pattern: Vec<(i32, i32)>,
}

#[derive(Component)]
struct MinoPatterns(Vec<MinoPattern>);

fn startup(commands: Commands) {
    println!("Mino plugin was loaded!");
}

fn register_mino_patterns(mut commands: Commands) {
    commands.insert_resource(MinoPatterns(vec![
        MinoPattern {
            mino_type: MinoTypes::I,
            pattern: vec![(0, 0), (0, -1), (0, 1), (0, 2)],
        },
        MinoPattern {
            mino_type: MinoTypes::L,
            pattern: vec![(0, 0), (0, -1), (0, 1), (-1, 1)],
        },
        MinoPattern {
            mino_type: MinoTypes::J,
            pattern: vec![(0, 0), (0, -1), (0, 1), (1, 1)],
        },
        MinoPattern {
            mino_type: MinoTypes::Z,
            pattern: vec![(0, 0), (0, -1), (1, 0), (1, 1)],
        },
        MinoPattern {
            mino_type: MinoTypes::S,
            pattern: vec![(0, 0), (1, 0), (0, 1), (1, -1)],
        },
        MinoPattern {
            mino_type: MinoTypes::O,
            pattern: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        },
        MinoPattern {
            mino_type: MinoTypes::T,
            pattern: vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
        },
    ]))
}

struct MinoManager {
    available: Vec<MinoTypes>,
}

impl MinoManager {

    fn new(&mut self) -> Self {
        MinoManager {
            available: vec![
            ],
        }
    }

    fn reset_available(&mut self) {
        self.available = vec![
            MinoTypes::I,
            MinoTypes::L,
            MinoTypes::J,
            MinoTypes::Z,
            MinoTypes::S,
            MinoTypes::O,
            MinoTypes::T,
        ];
    }

    fn consume(&mut self, types: MinoTypes) {
        if let Some(target_idx) = self.available.iter().position(|mt| *mt == types) {
            self.available.remove(target_idx);
        }
        if self.available.len() == 0 {
            self.reset_available();
        }
    }
}

struct MinoSheet(Handle<TextureAtlas>);

fn load_sprites(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = assets.load("sheet/minos.png");
    let atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(16.0), 7, 1);
    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(MinoSheet(atlas_handle));
}

fn spawn_block(
    commands: &mut Commands,
    mino_sheet: &Res<MinoSheet>,
    index: usize,
    position: Position,
) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.custom_size = Some(Vec2::splat(MINO_SIZE as f32));
    sprite.index = index;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: mino_sheet.0.clone(),
            ..Default::default()
        })
        .insert(position);
}

fn spawn_mino_one_loop(
    mut commands: Commands,
    mino_sheet: Res<MinoSheet>,
    patterns: Res<MinoPatterns>,
) {
    for i in 0..7 {
        let new_block = next_mino(&patterns.0);

        // dbg!("{:?}", &new_block);

        let init_x = X_LEN / 2;
        let init_y = Y_LEN + 5;

        new_block.pattern.iter().for_each(|(r_x, r_y)| {
            spawn_block(
                &mut commands,
                &mino_sheet,
                new_block.mino_type as usize,
                Position {
                    x: init_x as i32 + r_x,
                    y: -(i * 3) + init_y as i32 + r_y,
                },
            )
        });
    }
}
fn spawn_mino(mut commands: Commands, mino_sheet: Res<MinoSheet>, patterns: Res<MinoPatterns>) {
    let new_block = next_mino(&patterns.0);

    let init_x = X_LEN / 2;
    let init_y = Y_LEN + 5;

    new_block.pattern.iter().for_each(|(r_x, r_y)| {
        spawn_block(
            &mut commands,
            &mino_sheet,
            new_block.mino_type as usize,
            Position {
                x: init_x as i32 + r_x,
                y: init_y as i32 + r_y,
            },
        )
    });
}

fn next_mino(patterns: &Vec<MinoPattern>) -> MinoPattern {}

fn position_transform(mut query: Query<(&Position, &mut Transform, &mut TextureAtlasSprite)>) {
    let orig_x = MINO_SIZE as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let orig_y = MINO_SIZE as i32 / 2 - SCREEN_HEIGHT as i32 / 2;

    query
        .iter_mut()
        .for_each(|(pos, mut transform, mut sprite)| {
            transform.translation = Vec3::new(
                (orig_x + pos.x as i32 * MINO_SIZE as i32) as f32,
                (orig_y + pos.y as i32 * MINO_SIZE as i32) as f32,
                0.0,
            );
            sprite.custom_size = Some(Vec2::splat(MINO_SIZE as f32))
        });
}
