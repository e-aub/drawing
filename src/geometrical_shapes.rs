//TODO: impplement associated function random
// implement new for all shapes
pub trait Drawable{
    fn draw(&self){

    }

    fn color(&self){

    }
}


pub trait Displayable{
    fn display(&self){

    }
}



// Triangle

pub struct Triangle(&Point, &Point, &Point);

impl Triangle{
pub fn new(x: &Point, y: &Point, z: &Point) -> Self{
    return Self(x,y,z)
}
}

// Rectangle
pub struct Rectangle(Point, Point, Point, Point);

impl Rectangle{

pub fn new(x: &Point, y: &Point) -> Self{
    Self(x,y, Point(x.0, y.1), Point(x.1, y.0))
}
}


// Point
pub struct Point(i32, i32);

impl Point{
pub fn new(x: i32, y: i32) -> Self{
    Self(x,y)
}
}


// Line
pub struct Line(Point, Point);

impl Line{
pub fn new(a: Point, b: Point) -> Self{
    Self(a,b)
}
}

// Circle Shape
pub struct Circle{
    center: Point,
    radius: i32,
}

impl Circle{
pub fn new(center: Point, radius: i32) -> Self{
    Self{center, radius}
}
}
