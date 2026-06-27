use bevy::prelude::*;
use shared::enums::RobotStatus;
use shared::models::robot::RobotPosition;

#[derive(Event)]
pub struct RobotMoveEvent(pub RobotPosition);

#[derive(Component)]
pub struct Robot {
    pub id: String,
    pub status: RobotStatus,
}

pub struct RobotPlugin {
    pub robot_id: String,
}

#[derive(Resource)]
pub struct RobotConfig {
    pub robot_id: String,
}

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RobotConfig {
            robot_id: self.robot_id.clone(),
        })
        .add_event::<RobotMoveEvent>()
        .add_systems(Startup, setup_robot)
        .add_systems(Update, handle_robot_movement);
    }
}

fn setup_robot(mut commands: Commands, config: Res<RobotConfig>) {
    commands.spawn((
        Robot {
            id: config.robot_id.clone(),
            status: RobotStatus::Idle,
        },
        Transform::default(),
        Visibility::default(),
    ));
}

fn handle_robot_movement(
    mut events: EventReader<RobotMoveEvent>,
    mut query: Query<&mut Transform, With<Robot>>,
) {
    for event in events.read() {
        for mut transform in query.iter_mut() {
            transform.translation.x = event.0.x as f32;
            transform.translation.y = event.0.y as f32;
        }
    }
}
