/*
    Enum is typically used in any programming language to represent
    a value that can be one of several possibal variants
    When define an enum in Rust, essentially creating a new type that
    can have several possible variant. Each variant can be associated with 
    it own set of data. Which allow represent complex data structure using a 
    single enum
    Enums can have associated method like struct
    Only struct can have name represent variable
*/
#[derive(PartialEq)] //cai nay dung de implement cac toan tu tuong ung == , >= cho struct
struct Point {
    x : u32,
    y : u32
}

#[derive(PartialEq)]
enum CarStatus {
    MovingUp (u32, u32, u32), // in case that does not need name for variable
    MovingDown {speed : u32},
    NotMoving (Point),
    NotWorking,
}

// method and associate function for Enum
struct Retangle {
    x : f32,
    y : f32,
    z : f32,
    w : f32
}

enum Shape {
    Circle { x : f32, y: f32, radius : f32 },
    Retangle (Retangle),
    Square(f32, f32, f32),
}

impl Shape {
    fn new_circle(x : f32, y:f32, radius: f32) -> Shape {
        Shape::Circle {x , y , radius}
    }
    fn new_retangle(x : f32, y : f32, z : f32, w : f32) ->Shape {
        Shape::Retangle(Retangle {x,y,z,w})
    }
    fn new_square(x : f32, y : f32, z : f32)->Self {
        Shape::Square(x,y,z)
    }
    fn area(&self) -> f32 {
        let mut rc = 0.0;
        match self {
            Shape::Circle {x,y,radius} => {
                rc = x + y + radius;
            }
            Shape::Retangle(rec) => {
                rc = rec.z*rec.w
            }
            _ => {rc = 1.0}
        }
        return rc;
    }
}

fn find_value(array : &[i32], target : i32) -> Option<i32> {
    for (index , value) in array.iter().enumerate(){
        if *value == target {
            return Some(index as i32);
        }
    }
    None
}

/* 
fn main (){
    let mut current_car_status = CarStatus::NotMoving(Point { x : 0, y : 0});
    current_car_status = CarStatus::MovingUp(100,67,32);
    // if car is moving up print its speed
    if current_car_status == CarStatus::NotWorking {
        println!("car is not working");
    }
    current_car_status = CarStatus::NotMoving(Point {x : 1, y : 1});
    current_car_status = CarStatus::MovingDown {speed : 100};
    match current_car_status {
        CarStatus::MovingUp(a,..) => {
            println!("car is moving up with speed of {}", a);
        }
        CarStatus::NotMoving(Point {x,y}) => {
            println!("car it not moving");
            println!("x = {}, y = {}", x, y);
        }
        CarStatus::MovingDown  {speed : x} if x > 10 => {
            println!("hello");
        }
        _ => println!("car is not moving up")
    }
    let new_share = Shape::new_circle(1.0,2.0,2.0);
    let r = new_share.area();
    /*
        Option<T> is enum that represent an optional value 
        It can either be Some(T) indicating that there is a value of type T or None (absence of value)
        Some(value) is used to create an instance of Option<T> enum that contains a value
    */
    let v1 = Some(29);
    let v2 : Option<String> = Some("hello".to_string());
    let v3 : Option<i32> = None;
    println!("{:?}", v1);
    
    let my_array = [1,2,3,4,5,6];
    let index = find_value(&my_array, 2);
    if let Some(i) = index {
        println!("Found at index {}", i);
        println!("{}", my_array[i as usize]);
    } else {
        println!("value is not found in array");
    }
}

*/