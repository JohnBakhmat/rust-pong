
use bevy::prelude::*;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .add_startup_system(setup)
        .add_system(print_ball_info)
        .add_system(ball_movement)
        .add_system(ball_limits)
        .run();
}


fn hello_world(){
    println!("Hello, world!");
}


fn spawn_ball(mut commands: Commands){
    commands.spawn((Ball{
        position: Vector2{
            x:0.0,
            y:0.0,
        },
        velocity: Vector2{
            x:-10.0,
            y:2.0,
        }},
        SpriteBundle {
            sprite: Sprite{
                color: Color::rgb(1.,1.,1.),
                custom_size: Some(Vec2::new(10.,10.)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
                  );
}

fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
    spawn_ball(commands);
}

fn ball_movement(mut query: Query<(&mut Ball,&mut Transform)>){
    let (mut ball,mut sprite) = query.single_mut();
    ball.update_position();
    sprite.translation.x = ball.position.x;
    sprite.translation.y = ball.position.y;

}

fn ball_limits(mut query: Query<&mut Ball>){
    let mut ball = query.get_single_mut().unwrap();

    let width = 600.;
    let height = 400.;
    if ball.position.y < -height || ball.position.y > height {
        ball.bounce(Directions::Up);
    }
    if ball.position.x < -width || ball.position.x > width {
        ball.bounce(Directions::Left);
    }

}

fn print_ball_info(ball: Query<&Ball>){
    let ball = ball.get_single().unwrap();
    println!("Ball position is: x:{} y:{}", ball.position.x, ball.position.y); 
}


struct Vector2{
    x:f32,
    y:f32,
}

#[derive(Component)]
struct Ball{
    position: Vector2,
    velocity: Vector2,
}

impl Ball {
    fn apply_velocity(&mut self, velocity:&Vector2){
        self.velocity.x = velocity.x;
        self.velocity.y = velocity.y;
    }

    fn update_position(&mut self){
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn bounce(&mut self, direction:Directions){
        let velocity_modifier = match direction{
            Directions::Up | Directions::Down => Vector2{x:1.,y:-1.},
            Directions::Left | Directions::Right => Vector2{x:-1.,y:1.},
        };
        self.velocity.x *= velocity_modifier.x;
        self.velocity.y *= velocity_modifier.y;
    }
}

enum Directions{
    Up,
    Right,
    Down,
    Left
}
