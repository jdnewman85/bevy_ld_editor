use bevy::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::{PresentMode, WindowMode, PrimaryWindow};

mod helpers;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Clickable;

fn clickable_sprites(
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut clickable_sprite_query: Query<(&GlobalTransform, &mut Sprite, &mut Handle<Image>), With<Clickable>>,
    mut clickable_atlas_sprite_query: Query<(&GlobalTransform, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Clickable>>,
) {
    // TODO Assumes single camera marked as MainCamera
    let (camera, camera_transform) = camera_q.single();

    let wnd = window_q.get_single().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let screen_ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(screen_ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        //TEMP Let's hack in some mouse collision checks
        for (transform, mut sprite, mut texture_handle) in clickable_sprite_query.iter_mut() {
            let Some(image) = images.get(&texture_handle.clone()) else {
                dbg!("BUG? Image err");
                continue;
            };
            let delta = world_pos - transform.translation().truncate();
            let delta_minus_image = delta.abs() - (image.size()/2.0);
            if delta_minus_image.max_element() <= 0.0 {//if delta_minus_image.is_negative_bitmask() == 0b11 {
                //dbg!(world_pos.x, world_pos.y);
                sprite.color = Color::rgba(1.0, 0.0, 0.0, 1.0);
                *texture_handle = asset_server.load("white_square_32.png");
            } else {
                sprite.color = Color::WHITE;
            }
        }

        // Atlas
        for (transform, mut sprite, texture_handle) in clickable_atlas_sprite_query.iter_mut() {
            let texture_atlas = texture_atlases.get(texture_handle).unwrap();
            let image = texture_atlas.textures[sprite.index];
            let delta = world_pos - transform.translation().truncate();
            let delta_minus_image = delta.abs() - (image.size()/2.0);
            if delta_minus_image.max_element() <= 0.0 {//if delta_minus_image.is_negative_bitmask() == 0b11 {
                //dbg!(world_pos.x, world_pos.y);
                sprite.color = Color::rgba(1.0, 0.0, 0.0, 1.0);
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            } else {
                sprite.color = Color::WHITE;
            }
        }
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //let _: Handle<Image> = asset_server.load("white_square_32.png");

    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("white_ball_32_alpha.png"),
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..default()
        })
        .insert(Clickable);

    let texture_handle = asset_server.load("atlas_32_96_alpha.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle{
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..default()
        })
        .insert(Clickable);
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1024.0, 768.0).into(),
                        title: "LD Editor".to_string(),
                        ..default()
                    }),
                    ..default()
                }
            )
        )
        .insert_resource(Msaa::Sample2)
        .add_startup_system(startup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(clickable_sprites)
        .add_system(helpers::camera::movement)
        .run();
}

