
#[derive(Debug, Default, Copy, Clone)]
//duoc su dung de tu gencode , cho phep struct co the su dung ow trang thai debu, defaul , copy va clone
struct Person {
    name : String,
    age : i32,
    phone_number : String
}

fn create_person(name: String, age : i32, phone_number : String) -> Person {
    return Person { name, age, phone_number };
}


struct Process {
    name : String,
    pid : u32,
    group: String,
}

struct Point {
    x : i32,
    y : i32
}

impl Point {
    fn distance (&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
    fn translate(&mut self , dx : i32, dy : i32){
        self.x += dx;
        self.y += dy;
    }
    fn create(crook : &(i32,i32)) -> Point {
        Point{ x : crook.0, y : crook.1}
    }
}

fn update_point(p: &mut Point, dx : i32, dy : i32){
    p.x += dx;
    p.y += dy;
}
/* 
fn main() {
    let process1 = Process {
        name : String::from("ping"),
        pid : 1234,
        group : String::from("Networking")
    };
    let process2 = Process {
        name : String::from("router"),
        ..process1
    };
    let process3 = Process {
        pid : 78910,
        group : String::from("Security"),
        .. process2
    };
    let a = (1,2,3);
    println!("{}", a.0);
    let mut points = Point {
                    x : 3,
                    y : 4 
                    };
    println!("{}",points.distance());
    points.translate(2,3);
    println!("{:?}", points);
    update_point(&mut points, 3,4);
    println!("{:?}", points);
    let a = (2,3);
    let p1 = Point::create(&a);
    println!("{:?}", p1);
    match p1 {
        Point {x : 30, y : 20 } => println!("true"),
        _ => println!("false")
    }
}

associate function is function that can call without initializing object 
it can be called by :: for example : String::new()    
or */