use std::f64::consts::PI;
use std::io::Write;

trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
    fn get_name(&self) -> String;
    fn get_definition(&self) -> String;
    fn get_statistics(&self) -> String;

    /* Lets a shape say whether it is also a Polygon, so the statistics
       pass can sum longest segments without knowing concrete types.
       Returns: Some(self as a Polygon) for polygons, None otherwise.
       Can go wrong: nothing; non-polygons simply keep this default. */
    fn as_polygon(&self) -> Option<&dyn Polygon> {
        None
    }
}

trait Polygon: Shape {
    fn get_longest_side_length(&self) -> f64;
    fn get_shortest_side_length(&self) -> f64;
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn get_cartesian_string(&self) -> String {
        format!("({:.2}, {:.2})", self.x, self.y)
    }

    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
    }

    fn get_perimeter(&self) -> f64 {
        ((self.p1.x - self.p2.x).abs() + (self.p1.y - self.p2.y).abs()) * 2.0
    }

    fn get_name(&self) -> String {
        String::from("Rectangle")
    }

    fn get_definition(&self) -> String {
        format!(
            "I'm a rectangle defined by points {} and {}.",
            self.p1.get_cartesian_string(),
            self.p2.get_cartesian_string()
        )
    }

    fn get_statistics(&self) -> String {
        format!(
            "I have area, perimeter, longest side, shortest side: {:.2} {:.2} {:.2} {:.2}",
            self.get_area(),
            self.get_perimeter(),
            self.get_longest_side_length(),
            self.get_shortest_side_length()
        )
    }

    fn as_polygon(&self) -> Option<&dyn Polygon> {
        Some(self)
    }
}

impl Polygon for Rectangle {
    fn get_longest_side_length(&self) -> f64 {
        let xl: f64 = (self.p1.x - self.p2.x).abs();
        let yl: f64 = (self.p1.y - self.p2.y).abs();

        xl.max(yl)
    }

    fn get_shortest_side_length(&self) -> f64 {
        let xl: f64 = (self.p1.x - self.p2.x).abs();
        let yl: f64 = (self.p1.y - self.p2.y).abs();

        xl.min(yl)
    }
}

// Rust has no inheritance, so a Square owns a Rectangle and delegates
// the math to it instead of re-deriving it.
struct Square {
    rect: Rectangle,
}

impl Square {
    fn new(corner: Point, side: f64) -> Square {
        let opposite: Point = Point {
            x: corner.x + side,
            y: corner.y + side,
        };

        Square {
            rect: Rectangle {
                p1: corner,
                p2: opposite,
            },
        }
    }
}

impl Shape for Square {
    fn get_area(&self) -> f64 {
        self.rect.get_area()
    }

    fn get_perimeter(&self) -> f64 {
        self.rect.get_perimeter()
    }

    fn get_name(&self) -> String {
        String::from("Square")
    }

    fn get_definition(&self) -> String {
        format!(
            "I'm a square with corner {} and side length {:.2}.",
            self.rect.p1.get_cartesian_string(),
            self.get_longest_side_length()
        )
    }

    fn get_statistics(&self) -> String {
        format!(
            "I have area, perimeter, side length: {:.2} {:.2} {:.2}",
            self.get_area(),
            self.get_perimeter(),
            self.get_longest_side_length()
        )
    }

    fn as_polygon(&self) -> Option<&dyn Polygon> {
        Some(self)
    }
}

impl Polygon for Square {
    fn get_longest_side_length(&self) -> f64 {
        self.rect.get_longest_side_length()
    }

    fn get_shortest_side_length(&self) -> f64 {
        self.rect.get_shortest_side_length()
    }
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    fn get_side_lengths(&self) -> [f64; 3] {
        [
            self.p1.distance_to(&self.p2),
            self.p2.distance_to(&self.p3),
            self.p3.distance_to(&self.p1),
        ]
    }
}

impl Shape for Triangle {
    // Shoelace formula: area straight from the three vertices.
    fn get_area(&self) -> f64 {
        ((self.p1.x * (self.p2.y - self.p3.y)
            + self.p2.x * (self.p3.y - self.p1.y)
            + self.p3.x * (self.p1.y - self.p2.y))
            / 2.0)
            .abs()
    }

    fn get_perimeter(&self) -> f64 {
        self.get_side_lengths().iter().sum()
    }

    fn get_name(&self) -> String {
        String::from("Triangle")
    }

    fn get_definition(&self) -> String {
        format!(
            "I'm a triangle defined by points {}, {} and {}.",
            self.p1.get_cartesian_string(),
            self.p2.get_cartesian_string(),
            self.p3.get_cartesian_string()
        )
    }

    fn get_statistics(&self) -> String {
        format!(
            "I have area, perimeter, longest side, shortest side: {:.2} {:.2} {:.2} {:.2}",
            self.get_area(),
            self.get_perimeter(),
            self.get_longest_side_length(),
            self.get_shortest_side_length()
        )
    }

    fn as_polygon(&self) -> Option<&dyn Polygon> {
        Some(self)
    }
}

impl Polygon for Triangle {
    fn get_longest_side_length(&self) -> f64 {
        self.get_side_lengths().iter().cloned().fold(f64::MIN, f64::max)
    }

    fn get_shortest_side_length(&self) -> f64 {
        self.get_side_lengths().iter().cloned().fold(f64::MAX, f64::min)
    }
}

struct Ellipse {
    origin: Point,
    x_radius: f64,
    y_radius: f64,
}

impl Shape for Ellipse {
    fn get_area(&self) -> f64 {
        PI * self.x_radius * self.y_radius
    }

    // Ramanujan's approximation for the circumference of an ellipse.
    // Reduces exactly to 2*PI*r when both radii match, so Circle can
    // reuse it unchanged.
    fn get_perimeter(&self) -> f64 {
        let a: f64 = self.x_radius;
        let b: f64 = self.y_radius;

        PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt())
    }

    fn get_name(&self) -> String {
        String::from("Ellipse")
    }

    fn get_definition(&self) -> String {
        format!(
            "I'm an ellipse with origin {}, x-radius {:.2} and y-radius {:.2}.",
            self.origin.get_cartesian_string(),
            self.x_radius,
            self.y_radius
        )
    }

    fn get_statistics(&self) -> String {
        format!(
            "I have area and perimeter: {:.2} {:.2}",
            self.get_area(),
            self.get_perimeter()
        )
    }
}

// Same idea as Square: a Circle is an Ellipse with equal radii.
struct Circle {
    ellipse: Ellipse,
}

impl Circle {
    fn new(origin: Point, radius: f64) -> Circle {
        Circle {
            ellipse: Ellipse {
                origin,
                x_radius: radius,
                y_radius: radius,
            },
        }
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        self.ellipse.get_area()
    }

    fn get_perimeter(&self) -> f64 {
        self.ellipse.get_perimeter()
    }

    fn get_name(&self) -> String {
        String::from("Circle")
    }

    fn get_definition(&self) -> String {
        format!(
            "I'm a circle with origin {} and radius {:.2}.",
            self.ellipse.origin.get_cartesian_string(),
            self.ellipse.x_radius
        )
    }

    fn get_statistics(&self) -> String {
        format!(
            "I have area and perimeter: {:.2} {:.2}",
            self.get_area(),
            self.get_perimeter()
        )
    }
}

// Returned when stdin runs out (EOF) while a value is still being
// requested, so the menu loop doesn't spin forever re-prompting a
// stream that can never supply more data.
struct InputExhausted;

/* Prints prompt, then reads one line from stdin.
   Returns: the trimmed line, or Err(InputExhausted) at EOF.
   Can go wrong: a read error is treated the same as EOF. */
fn read_line(prompt: &str) -> Result<String, InputExhausted> {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();

    let mut line: String = String::new();

    match std::io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => Err(InputExhausted),
        Ok(_) => Ok(String::from(line.trim())),
    }
}

/* Reads a number, re-prompting until the input parses.
   Returns: the number entered, or Err(InputExhausted) at EOF.
   Can go wrong: only EOF; bad input just re-prompts. */
fn read_number(prompt: &str) -> Result<f64, InputExhausted> {
    loop {
        let line: String = read_line(prompt)?;

        match line.parse::<f64>() {
            Ok(value) if value.is_finite() => return Ok(value),
            _ => println!("Invalid input. Please enter a number."),
        }
    }
}

fn read_positive_number(prompt: &str) -> Result<f64, InputExhausted> {
    loop {
        let value: f64 = read_number(prompt)?;

        if value > 0.0 {
            return Ok(value);
        }

        println!("Please enter a value greater than 0.");
    }
}

fn read_choice(prompt: &str, lo: u32, hi: u32) -> Result<u32, InputExhausted> {
    loop {
        let line: String = read_line(prompt)?;

        match line.parse::<u32>() {
            Ok(value) if value >= lo && value <= hi => return Ok(value),
            _ => println!(
                "Invalid choice. Please enter a whole number between {} and {}.",
                lo, hi
            ),
        }
    }
}

fn read_point(label: &str) -> Result<Point, InputExhausted> {
    let x: f64 = read_number(&format!("Enter {} x: ", label))?;
    let y: f64 = read_number(&format!("Enter {} y: ", label))?;

    Ok(Point { x, y })
}

fn read_rectangle() -> Result<Rectangle, InputExhausted> {
    loop {
        let p1: Point = read_point("first corner")?;
        let p2: Point = read_point("opposite corner")?;

        if p1.x != p2.x && p1.y != p2.y {
            return Ok(Rectangle { p1, p2 });
        }

        println!("Those corners don't make a rectangle (zero width or height). Try again.");
    }
}

fn read_square() -> Result<Square, InputExhausted> {
    let corner: Point = read_point("bottom-left corner")?;
    let side: f64 = read_positive_number("Enter side length: ")?;

    Ok(Square::new(corner, side))
}

fn read_triangle() -> Result<Triangle, InputExhausted> {
    loop {
        let triangle: Triangle = Triangle {
            p1: read_point("first vertex")?,
            p2: read_point("second vertex")?,
            p3: read_point("third vertex")?,
        };

        if triangle.get_area() > 0.0 {
            return Ok(triangle);
        }

        println!("Those points are in a line and can't form a triangle. Try again.");
    }
}

fn read_ellipse() -> Result<Ellipse, InputExhausted> {
    let origin: Point = read_point("origin")?;
    let x_radius: f64 = read_positive_number("Enter x-radius: ")?;
    let y_radius: f64 = read_positive_number("Enter y-radius: ")?;

    Ok(Ellipse {
        origin,
        x_radius,
        y_radius,
    })
}

fn read_circle() -> Result<Circle, InputExhausted> {
    let origin: Point = read_point("origin")?;
    let radius: f64 = read_positive_number("Enter radius: ")?;

    Ok(Circle::new(origin, radius))
}

fn print_menu() {
    println!();
    println!("===== Shape Summarizer =====");
    println!("1. Add Rectangle");
    println!("2. Add Square");
    println!("3. Add Triangle");
    println!("4. Add Ellipse");
    println!("5. Add Circle");
    println!("6. Finish and show statistics");
}

/* Runs the menu until the user finishes, pushing each new shape into
   the collection as it is built.
   Returns: Ok(()) when the user picks "finish", Err(InputExhausted) if
   stdin runs out first. Shapes already added stay in the collection
   either way. */
fn run_menu(shapes: &mut Vec<Box<dyn Shape>>) -> Result<(), InputExhausted> {
    loop {
        print_menu();

        let new_shape: Box<dyn Shape> = match read_choice("Enter your choice: ", 1, 6)? {
            1 => Box::new(read_rectangle()?),
            2 => Box::new(read_square()?),
            3 => Box::new(read_triangle()?),
            4 => Box::new(read_ellipse()?),
            5 => Box::new(read_circle()?),
            _ => return Ok(()),
        };

        println!("Added a new {}.", new_shape.get_name());
        shapes.push(new_shape);
    }
}

// Generic over anything that implements Shape; ?Sized lets it accept
// the trait objects stored in the Vec as well as concrete shapes.
fn print_shape_details<T: Shape + ?Sized>(s: &T) {
    println!("{}", s.get_definition());
    println!("{}", s.get_statistics());
}

/* Walks every shape collected and totals area, perimeter, and the
   longest segment of every Polygon.
   Accepts: the shapes entered by the user.
   Returns: nothing; prints each shape and the grand totals.
   Can go wrong: nothing -- an empty collection prints zero totals. */
fn print_statistics(shapes: &[Box<dyn Shape>]) {
    let mut total_area: f64 = 0.0;
    let mut total_perimeter: f64 = 0.0;
    let mut total_longest_segment: f64 = 0.0;
    let mut polygon_count: usize = 0;

    println!();
    println!("===== Final Statistics =====");

    for shape in shapes {
        print_shape_details(&**shape);

        total_area += shape.get_area();
        total_perimeter += shape.get_perimeter();

        if let Some(polygon) = shape.as_polygon() {
            total_longest_segment += polygon.get_longest_side_length();
            polygon_count += 1;
        }
    }

    println!();
    println!("Shapes entered: {} ({} polygons)", shapes.len(), polygon_count);
    println!("Total area: {:.2}", total_area);
    println!("Total perimeter: {:.2}", total_perimeter);
    println!(
        "Sum of perimeter and area of every shape: {:.2}",
        total_area + total_perimeter
    );
    println!(
        "Sum of the longest segment of every polygon: {:.2}",
        total_longest_segment
    );
}

fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

    if run_menu(&mut shapes).is_err() {
        println!();
        println!("No more input available; wrapping up.");
    }

    print_statistics(&shapes);
}
