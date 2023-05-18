
use bevy::{prelude::*, utils::petgraph::Direction};

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .add_startup_system(setup)
        .add_system(print_ball_info)
        .add_system(ball_movement)
        .add_system(ball_limits)
        .add_system(paddle_movement)
        .add_system(hit_ball)
        .run();
}


fn hello_world(){
    println!("Hello, world!");
}


fn spawn_ball(commands:&mut Commands){
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


fn spawn_players(commands:&mut Commands){
    commands.spawn((
            SpriteBundle{
                sprite: Sprite{
                    color: Color::rgb(1.,1.,1.),
                    custom_size: Some(Vec2::new(20.,120.)),
                    ..default()
                },
                transform: Transform::from_xyz(-500.0, 0.0, 0.0),
                ..default()   
            },
            Paddle{
                speed:10.0,
                position:Vector2{x:-500.0,y:0.0},
                is_player:true
            }
            ));
    commands.spawn((
            SpriteBundle{
                sprite: Sprite{
                    color: Color::rgb(1.,1.,1.),
                    custom_size: Some(Vec2::new(20.,120.)),
                    ..default()
                },
                transform: Transform::from_xyz(500.0, 0.0, 0.0),
                ..default()   
            },
            Paddle{

                speed:10.0,
                position:Vector2{x:500.0,y:0.0},
                is_player:false
            }
            ));
}

fn paddle_movement(keyboard_input:Res<Input<KeyCode>>, mut query: Query<(&mut Paddle, &mut Transform)>){

   for (mut paddle,mut transform) in query.iter_mut(){
        if paddle.is_player {
            if keyboard_input.pressed(KeyCode::W) {
                paddle.move_paddle(Directions::Up);
            }
            if keyboard_input.pressed(KeyCode::S) {
                paddle.move_paddle(Directions::Down);
            }
            transform.translation.y = paddle.position.y;
        };
   } 

}

fn hit_ball(paddles:Query<&mut Paddle>, mut balls:Query<&mut Ball>) {
    let mut ball = balls.get_single_mut().unwrap();
    for paddle in paddles.iter(){
        if ball.position.x == paddle.position.x
           && ball.position.y < (paddle.position.y + 120.0) && ball.position.y > (paddle.position.y - 120.0)
           {
            ball.bounce(Directions::Right);
           }

    }
}


fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
    let commands = &mut commands;
    spawn_ball(commands);
    spawn_players(commands);
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
struct Paddle{
    position:Vector2,
    speed:f32,
    is_player:bool
}

impl Paddle{
    fn move_paddle(&mut self, direction: Directions){
        match direction{
            Directions::Up => self.position.y += self.speed,
            Directions::Down => self.position.y -= self.speed,
            _ => {}
        };
    }
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
