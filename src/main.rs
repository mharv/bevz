use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // Uncomment this to override the default log settings:
            // level: bevy::log::Level::TRACE,
            // filter: "wgpu=warn,bevy_ecs=info".to_string(),
            ..default()
        }))
        .add_plugins(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource, Debug)]
struct GreetTimer(Timer, &'static str);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("ola {}!", name.0);
            info!("trace");
        }
    }
    println!("{}", timer.0.times_finished_this_tick());
    println!("{}", timer.0.remaining_secs());
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(
            Timer::from_seconds(3., TimerMode::Repeating),
            "greet_timer",
        ))
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people);
    }
}
