use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};

mod ui;
use ui::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PostUpdate, update_layout)
        .add_system_to_stage(CoreStage::PostUpdate, sync_rect_transform_system)
        .add_system_to_stage(CoreStage::PostUpdate, update_sprite_size)
        .run();
}

fn setup(windows: Res<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    let window_width = window.width();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let root_node = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 0,
        ..Default::default()
    })
    .insert(UINode {
        style: Box::new(Row {
            width: window_width,
        })
    }).id();

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Expanded {
            flex: 1.0
        })
    }).insert(Parent(root_node));

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::ZERO),
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Expanded {
            flex: 2.0
        })
    }).insert(Parent(root_node));

    let expanded_container = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Expanded {
            flex: 3.0
        })
    }).insert(Parent(root_node)).id();

    let padded_container = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Padding {
            top: 10.0,
            bottom: 10.0,
            left: 10.0,
            right: 10.0
        })
    }).insert(Parent(expanded_container)).id();

    let row_container = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::PINK,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Row {
            width: 1000.0
        })
    }).insert(Parent(padded_container)).id();

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::PURPLE,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Expanded {
            flex: 1.0
        })
    }).insert(Parent(row_container)).id();

    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        sprite: Sprite {
            color: Color::YELLOW,
            custom_size: Some(Vec2::ZERO),
            ..Default::default()
        },
        ..Default::default()
    }).insert(RectTransform {
        depth: 1,
        ..Default::default()
    }).insert(UINode {
        style: Box::new(Expanded {
            flex: 1.0
        })
    }).insert(Parent(row_container));
}

fn update_sprite_size(mut node_query: Query<(&RectTransform, &mut Sprite)>) {
    for (rect_transform, mut sprite) in node_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(rect_transform.size.width, rect_transform.size.height));
    }
}