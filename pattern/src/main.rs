enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}

struct Point{
    x: i32,
    y: i32,
}

fn process_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y } => println!("({},{})", x, y),
    }
}

fn main(){
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(2.0, 3.0);
    let square = Shape::Square(2.0);
    println!("{}", calculate_area(&circle));
    println!("{}", calculate_area(&rectangle));
    println!("{}", calculate_area(&square));

    let area = match circle {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };
    println!("{}", area);

    let point1 = Point { x: 0, y: 0 };
    let point2 = Point { x: 0, y: 5 };
    process_point(point1);
    process_point(point2);

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}