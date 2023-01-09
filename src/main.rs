use bevy::{prelude::*, time::FixedTimestep, window::PresentMode};
use rubiks::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rubiks Cube!".to_string(),
                width: 100.,
                height: 100.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_cube)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(update_cubelets),
        )
        .add_system(turn_input)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_cube(mut commands: Commands) {
    commands
        .spawn((
            RubiksCube { cube: Cube::new() },
            TransformBundle { ..default() },
            VisibilityBundle { ..default() },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    RubiksCubeFace {
                        face: RubiksFace::L,
                    },
                    TransformBundle {
                        local: Transform {
                            translation: Vec3 {
                                x: -30.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    VisibilityBundle { ..default() },
                ))
                .with_children(|face_parent| {
                    spawn_faces(face_parent, rubiks::Color::Orange);
                });

            parent
                .spawn((
                    RubiksCubeFace {
                        face: RubiksFace::F,
                    },
                    TransformBundle {
                        local: Transform {
                            translation: Vec3 {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    VisibilityBundle { ..default() },
                ))
                .with_children(|face_parent| {
                    spawn_faces(face_parent, rubiks::Color::Green);
                });

            parent
                .spawn((
                    RubiksCubeFace {
                        face: RubiksFace::R,
                    },
                    TransformBundle {
                        local: Transform {
                            translation: Vec3 {
                                x: 30.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    VisibilityBundle { ..default() },
                ))
                .with_children(|face_parent| {
                    spawn_faces(face_parent, rubiks::Color::Red);
                });

            parent
                .spawn((
                    RubiksCubeFace {
                        face: RubiksFace::B,
                    },
                    TransformBundle {
                        local: Transform {
                            translation: Vec3 {
                                x: 60.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    VisibilityBundle { ..default() },
                ))
                .with_children(|face_parent| {
                    spawn_faces(face_parent, rubiks::Color::Blue);
                });

            parent
                .spawn((
                    RubiksCubeFace {
                        face: RubiksFace::U,
                    },
                    TransformBundle {
                        local: Transform {
                            translation: Vec3 {
                                x: 0.0,
                                y: 30.0,
                                z: 0.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    VisibilityBundle { ..default() },
                ))
                .with_children(|face_parent| {
                    spawn_faces(face_parent, rubiks::Color::White);
                });

            parent
                .spawn((
                    RubiksCubeFace {
                        face: RubiksFace::D,
                    },
                    TransformBundle {
                        local: Transform {
                            translation: Vec3 {
                                x: 0.0,
                                y: -30.0,
                                z: 0.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    VisibilityBundle { ..default() },
                ))
                .with_children(|face_parent| {
                    spawn_faces(face_parent, rubiks::Color::Yellow);
                });
        });
}

fn spawn_faces(child_builder: &mut ChildBuilder, color: rubiks::Color) {
    let bevy_color = map_colors(color);
    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: -10.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::M_L,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::M_M,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::M_R,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: -10.0,
                    y: 10.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::T_L,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 10.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::T_M,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 10.0,
                    y: 10.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::T_R,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: -10.0,
                    y: -10.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::B_L,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: -10.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::B_M,
        },
    ));

    child_builder.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 10.0,
                    y: -10.0,
                    z: 0.0,
                },
                ..default()
            },
            sprite: Sprite {
                color: bevy_color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        RubiksCubeSpace {
            cubelet: RubiksCubelet::B_R,
        },
    ));
}

fn update_cubelets(
    q_cube: Query<(&RubiksCube, &Children)>,
    q_faces: Query<(&RubiksCubeFace, &Children)>,
    mut q_cubelets: Query<(&RubiksCubeSpace, &mut Sprite)>,
) {
    for (cube, faces) in q_cube.iter() {
        for child in faces.iter() {
            if let Ok((face, cubelets)) = q_faces.get(*child) {
                for face_child in cubelets.iter() {
                    if let Ok((cubelet, mut sprite)) = q_cubelets.get_mut(*face_child) {
                        let cube_side = face.face.get_side_from_cube(&cube.cube);

                        sprite.color =
                            map_colors(cubelet.cubelet.get_face_color_cubelet(cube_side));
                    }
                }
            }
        }
    }
}

#[derive(Component)]
struct RubiksCube {
    cube: Cube,
}

#[derive(Component)]
struct RubiksCubeFace {
    face: RubiksFace,
}

#[derive(Component)]
struct RubiksCubeSpace {
    cubelet: RubiksCubelet,
}

enum RubiksFace {
    L,
    F,
    R,
    B,
    U,
    D,
}

enum RubiksCubelet {
    T_L,
    T_M,
    T_R,
    M_L,
    M_M,
    M_R,
    B_L,
    B_M,
    B_R,
}

impl RubiksCubelet {
    fn get_face_color_cubelet(&self, face: &Face) -> rubiks::Color {
        match self {
            RubiksCubelet::T_L => face.t_l,
            RubiksCubelet::T_M => face.t_m,
            RubiksCubelet::T_R => face.t_r,
            RubiksCubelet::M_L => face.m_l,
            RubiksCubelet::M_M => face.m_m,
            RubiksCubelet::M_R => face.m_r,
            RubiksCubelet::B_L => face.b_l,
            RubiksCubelet::B_M => face.b_m,
            RubiksCubelet::B_R => face.b_r,
        }
    }
}

fn map_colors(color: rubiks::Color) -> bevy::prelude::Color {
    match color {
        rubiks::Color::Orange => ORANGE,
        rubiks::Color::Green => GREEN,
        rubiks::Color::Red => RED,
        rubiks::Color::Blue => BLUE,
        rubiks::Color::White => WHITE,
        rubiks::Color::Yellow => YELLOW,
    }
}

const BLUE: bevy::prelude::Color = bevy::prelude::Color::rgb(0.25, 0.25, 0.75);
const RED: bevy::prelude::Color = bevy::prelude::Color::rgb(1.0, 0.0, 0.0);
const GREEN: bevy::prelude::Color = bevy::prelude::Color::rgb(0.25, 1.00, 0.0);
const YELLOW: bevy::prelude::Color = bevy::prelude::Color::rgb(1.0, 1.0, 0.0);
const WHITE: bevy::prelude::Color = bevy::prelude::Color::rgb(1.0, 1.0, 1.0);
const ORANGE: bevy::prelude::Color = bevy::prelude::Color::rgb(1.00, 0.5, 0.00);

impl RubiksFace {
    pub fn get_side_from_cube<'a>(&self, cube: &'a Cube) -> &'a Face {
        match self {
            RubiksFace::L => &cube.l,
            RubiksFace::F => &cube.f,
            RubiksFace::R => &cube.r,
            RubiksFace::B => &cube.b,
            RubiksFace::U => &cube.u,
            RubiksFace::D => &cube.d,
        }
    }
}

fn turn_input(keys: Res<Input<KeyCode>>, mut q_cube: Query<&mut RubiksCube>) {
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        if keys.just_pressed(KeyCode::F) {
            for mut cube in &mut q_cube {
                cube.cube.f_p();
            }
        }
        if keys.just_pressed(KeyCode::R) {
            for mut cube in &mut q_cube {
                cube.cube.r_p();
            }
        }
        if keys.just_pressed(KeyCode::L) {
            for mut cube in &mut q_cube {
                cube.cube.l_p();
            }
        }
        if keys.just_pressed(KeyCode::U) {
            for mut cube in &mut q_cube {
                cube.cube.u_p();
            }
        }
        if keys.just_pressed(KeyCode::B) {
            for mut cube in &mut q_cube {
                cube.cube.b_p();
            }
        }
        if keys.just_pressed(KeyCode::D) {
            for mut cube in &mut q_cube {
                cube.cube.d_p();
            }
        }
    } else {
        if keys.just_pressed(KeyCode::F) {
            for mut cube in &mut q_cube {
                cube.cube.f();
            }
        }
        if keys.just_pressed(KeyCode::R) {
            for mut cube in &mut q_cube {
                cube.cube.r();
            }
        }
        if keys.just_pressed(KeyCode::L) {
            for mut cube in &mut q_cube {
                cube.cube.l();
            }
        }
        if keys.just_pressed(KeyCode::U) {
            for mut cube in &mut q_cube {
                cube.cube.u();
            }
        }
        if keys.just_pressed(KeyCode::B) {
            for mut cube in &mut q_cube {
                cube.cube.b();
            }
        }
        if keys.just_pressed(KeyCode::D) {
            for mut cube in &mut q_cube {
                cube.cube.d();
            }
        }
    }
}
