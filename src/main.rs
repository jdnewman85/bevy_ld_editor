use bevy::prelude::*;
use bevy_render::camera::RenderTarget;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
//use bevy::window::{PresentMode, WindowMode};

mod helpers;

use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Clickable;

fn clickable_sprites(
    wnds: Res<Windows>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    assets: Res<Assets<Image>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut clickable_sprite_query: Query<(&GlobalTransform, &mut Sprite, &mut Handle<Image>), With<Clickable>>,
    mut clickable_atlas_sprite_query: Query<(&GlobalTransform, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Clickable>>,
) {
    // TODO Assumes single camera marked as MainCamera
    let (camera, camera_transform) = camera_q.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let screen_ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(screen_ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        //TEMP Let's hack in some mouse collision checks
        for (transform, mut sprite, mut texture_handle) in clickable_sprite_query.iter_mut() {
            let image = assets.get(texture_handle.clone()).unwrap();
            let image_size = image.size();
            let x1 = transform.translation.x - (image_size.x/2.0);
            let y1 = transform.translation.y - (image_size.y/2.0);
            let x2 = transform.translation.x + (image_size.x/2.0);
            let y2 = transform.translation.y + (image_size.y/2.0);

            if world_pos.x > x1 && world_pos.x < x2 &&
                world_pos.y > y1 && world_pos.y < y2
            {
                //dbg!(world_pos.x, world_pos.y);
                sprite.color = Color::Rgba{
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                };
                *texture_handle = asset_server.load("white_square_32.png");
            } else {
                sprite.color = Color::WHITE;
            }
        }

        // Atlas
        for (transform, mut sprite, texture_handle) in clickable_atlas_sprite_query.iter_mut() {
            let texture_atlas = texture_atlases.get(texture_handle).unwrap();
            let mut image_size = texture_atlas.size;
            image_size.x /= 3.0; //TODO BUG This is full size of underlying atlas image
            let x1 = transform.translation.x - (image_size.x/2.0);
            let y1 = transform.translation.y - (image_size.y/2.0);
            let x2 = transform.translation.x + (image_size.x/2.0);
            let y2 = transform.translation.y + (image_size.y/2.0);

            if world_pos.x > x1 && world_pos.x < x2 &&
                world_pos.y > y1 && world_pos.y < y2
            {
                //dbg!(world_pos.x, world_pos.y);
                sprite.color = Color::Rgba{
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                };
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
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("white_ball_32_alpha.png"),
            transform: Transform::from_xyz(300.0, 0.0, 0.0),
            ..default()
        })
        .insert(Clickable);

    let texture_handle = asset_server.load("atlas_32_96_alpha.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle{
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..default()
        })
        .insert(Clickable);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Sprite based ladder editor"),
            width: 1270.0,
            height: 720.0,
//          mode: WindowMode::Fullscreen,
//          present_mode: PresentMode::Immediate, //TODO TEMP request disable vsync
            ..Default::default()
        })
        .insert_resource(Msaa{samples: 1})
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(clickable_sprites)
        //FPS
//      .add_plugin(LogDiagnosticsPlugin::default())
//      .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(helpers::camera::movement)
        .run();
}

