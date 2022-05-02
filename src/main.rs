use std::f32::consts::PI;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

enum Color {
    Red, Green, Blue
}

enum Shape {
    Rectangle {width: f32, height: f32},
    Circle{radius : f32},
    Square {side: f32}
}

fn color_to_string(c: Color) {
    let s = match c {
        Color::Red => "Rouge",
        Color::Green => "Vert",
        Color::Blue => "Bleu"
    };
    println!("La couleur passée en paramètre est le {}", s);
}

fn distance(p0: Point, p1: Point) -> i32{
    sq(p0.x - p1.x) + sq(p0.y - p1.y) + sq(p0.z - p1.z)
}

fn shape_area(s: Shape) -> f32 {
    match s {
        Shape::Rectangle{width, height} => width * height,
        Shape::Square{side} => side * side,
        Shape::Circle { radius } => PI * radius * radius
    }
}

fn sq(x: i32) -> i32 {
    x * x
}

// Multiply Function
fn multiply(a: i32, b: i32) -> i32 {
    a*b
}

// Factorial Recursive method
fn factorial(i:i32) -> i32
{
    if i == 0
    {
        1
    } else {
        i * factorial(i -1)
    }
}

// Factorial Method with While
fn factorial_while(mut i: i32) -> i32 {
    let mut resultat = 1;

    while i>0 {
        resultat *= i;
        i = i - 1;
    }

    resultat
}

// Factorial Method with For
fn factorial_for(mut i: i32) -> i32 {
    let mut resultat = 1;
    for n in  1..(i+1){
        resultat *= n;
    }

    resultat
}

// Factorial Method: Rust version
fn factorial_rust(mut i: i32) -> i32 {
    (1..(i+1)).fold(1,multiply)
}

// Factorial Method: Rust version 2
fn factorial_rust2(mut i: i32) -> i32 {
    (1..(i+1)).fold(1,|a,b| a * b)
}

// Factorial Method: Rust version 3
fn factorial_rust3(mut i: i32) -> i32 {
    (1..(i+1)).product()
}

fn main() {
    println!("\n----- Création de méthodes -----");
    println!("Factorial Method: Rust version 3");
    for i in 1..10 {
        println!("-\tfactorial_rust3({}): {}",i, factorial_rust3(i));
    }

    println!("\n----- Création de structures -----");
    let p0 = Point{x:1, y:2, z:3};
    let p1 = Point{x:5, y:2, z:3};

    println!("Mon point est: {p0:?}");
    println!("Mon point est: {p1:?}");
    println!("Ma distance est de {}", distance(p0, p1));

    println!("\n----- Utilisation d'enumération -----");
    let c = Color::Blue;
    color_to_string(c);

    let _s0 = Shape::Rectangle { width: 2.0, height: 3.0 };
    let _s1 = Shape::Square { side: 2.0 };
    let _s2 = Shape::Circle { radius: 3.0 };

    println!("Surface calculée: {}", shape_area(_s2));

}
