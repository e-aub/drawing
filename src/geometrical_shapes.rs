use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);

    fn color(&self) -> Color {
        let r = rand::thread_rng().gen_range(250..=255);
        let g = rand::thread_rng().gen_range(250..=255);
        let b = rand::thread_rng().gen_range(250..=255);

        Color { r, g, b, a: 255 }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(1..width);
        let y = rand::thread_rng().gen_range(1..height);

        Self::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color());
    }
}

// Line
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line(Point, Point);

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self(a, b)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a = Point::random(width, height);
        let b = Point::random(width, height);
        Self::new(a, b)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let m = (self.1.1 - self.0.1) / (self.1.0 - self.0.0);
        let b = self.0.1 - (m * self.0.0);
        let color = self.color();

        let min = self.0.0.min(self.1.0);
        let max = self.0.0.max(self.1.0);
        for x in min..max {
            let y = m * x + b;
            image.display(x, y, color.clone());
        }
    }
}

// Rectangle
pub struct Rectangle(Point, Point, Point, Point);

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(
            dbg!(*p1),
            dbg!(*p2),
            dbg!(Point(p1.0, p2.0)),
            dbg!(Point(p2.0, p1.0)),
        )
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.0, self.2).draw(image);
        Line::new(self.2, self.1).draw(image);
        Line::new(self.1, self.3).draw(image);
        Line::new(self.3, self.0).draw(image);
    }
}

// Circle Shape
// pub struct Circle {
//     center: Point,
//     radius: i32,
// }

// impl Circle {
//     pub fn new(center: Point, radius: i32) -> Self {
//         Self { center, radius }
//     }
// }

// Triangle

// pub struct Triangle(&Point, &Point, &Point);

// impl Triangle {
//     pub fn new(x: &Point, y: &Point, z: &Point) -> Self {
//         return Self(x, y, z);
//     }
// }
