use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);

    fn color(&self) -> Color {
        let r = rand::thread_rng().gen_range(0..=255);
        let g = rand::thread_rng().gen_range(0..=255);
        let b = rand::thread_rng().gen_range(0..=255);

        Color { r, g, b, a: 255 }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::thread_rng().gen_range(1..width);
        let y = rand::thread_rng().gen_range(1..height);

        Self::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
}

// Line
#[derive(Debug, Clone)]
pub struct Line {
    start: Point,
    end: Point,
    color: Color,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a = Point::random(width, height);
        let b = Point::random(width, height);
        Self::new(a, b)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x_start = self.start.x;
        let y_start = self.start.y;
        let x_end = self.end.x;
        let y_end = self.end.y;

        let dx = (x_end - x_start) as f32;
        let dy = (y_end - y_start) as f32;

        let steps = dx.abs().max(dy.abs()) as i32;

        let x_increment = dx / steps as f32;
        let y_increment = dy / steps as f32;

        let mut x = x_start as f32;
        let mut y = y_start as f32;

        for _ in 0..=steps {
            image.display(x.round() as i32, y.round() as i32, self.color.clone());
            x += x_increment;
            y += y_increment;
        }
    }
}

// Rectangle
pub struct Rectangle {
    point_a: Point,
    point_b: Point,
    point_c: Point,
    point_d: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            point_a: *p1,
            point_b: *p2,
            point_c: Point { x: p1.x, y: p2.y },
            point_d: Point { x: p2.x, y: p1.y },
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.point_a, self.point_c).draw(image);
        Line::new(self.point_c, self.point_b).draw(image);
        Line::new(self.point_b, self.point_d).draw(image);
        Line::new(self.point_d, self.point_a).draw(image);
    }
}

// Triangle
pub struct Triangle {
    point_a: Point,
    point_b: Point,
    point_c: Point,
}

impl Triangle {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Self {
        Self {
            point_a: *point_a,
            point_b: *point_b,
            point_c: *point_c,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.point_a, self.point_c).draw(image);
        Line::new(self.point_c, self.point_b).draw(image);
        Line::new(self.point_b, self.point_a).draw(image);
    }
}

// // Circle Shape
// // pub struct Circle {
// //     center: Point,
// //     radius: i32,
// // }

// // impl Circle {
// //     pub fn new(center: Point, radius: i32) -> Self {
// //         Self { center, radius }
// //     }
// // }


pub struct Cube {
    rec_1: Rectangle,
    rec_2: Rectangle,
}

impl Cube {
    pub fn new(a: &Point, b: &Point) -> Self{
        let dx = (a.x - b.x) / 2;
        let dy = -((a.y - b.y)/2);
        Self{
            rec_1: Rectangle::new(&a, &b),
            rec_2 : Rectangle::new(&Point { x: (a.x + dx), y: (a.y + dy) }, &Point { x: (b.x + dx), y: (b.y + dy) })
        }
    }
}


impl Drawable for Cube{
    fn draw(&self, image :&mut Image){
        self.rec_1.draw(image);
        self.rec_2.draw(image);
    }
}