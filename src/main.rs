use bevy::prelude::*;
use bevy_render::camera::RenderTarget;

use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Clickable;

fn clickable_sprites(
    wnds: Res<Windows>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut clickable_sprite_query: Query<(&GlobalTransform, &mut Sprite, &mut Handle<Image>), With<Clickable>>,
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
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Sprite based ladder editor"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(clickable_sprites)
        .run();
}

