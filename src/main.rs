use termion::cursor::Goto;
use std::vec;

// implements Write for stdout allowing flush()
use std::io::Write;

// print the text at a certain place
macro_rules! print_at {
    ( $x:expr, $y:expr, $( $f:expr ),* ) => {
        println!("{}{}", Goto($x, $y), format!($( $f, )*))
    };
}

#[derive(Clone, Copy)]
struct Point(i32, i32, i32);

#[derive(Clone, Copy)]
struct Line(Point, Point);

// rotate one point
impl Point {
    fn rotate(&mut self, multiplier: Point) {
        // use miltiplier to generate a matrix 
        // this makes it so much easier to rotate a point
        // the user can specify x, y, and z rotations individually and it is easier to implement
    }
}

impl Line {
    fn project(self, focal: i32) -> Line {
        // project it to 2d

        Line(Point(0, 0, 0), Point(0, 0, 0))
    }

    // rotate each child point
    fn rotate(&mut self, multiplier: Point) {
        self.0.rotate(multiplier);
        self.1.rotate(multiplier);
    }

    fn draw(self) {
        // error check to make sure we have a 2d (projected) line
        // draw
    }
}

// prompt user for input
// returns user input as int
fn prompt_int(prompt: &str) -> i32 {
    print!("{}", prompt);
    std::io::stdout().flush().expect("Could not flush output...");

    let mut buf: String = String::new();

    // get input
    std::io::stdin().read_line(&mut buf).expect("Could not read input...");

    // convert and return input
    buf.trim().parse::<i32>().expect("Please input integer value...")
}

fn main() {

    // list of lines to render
    let mut vec: Vec<Line> = vec!();

    // get user input to populate the lines
    // no decimals because we are working in the terminal
    // TODO

    // get focal point
    let focal_distance: i32 = prompt_int("Focal Distance: ");

    // DEBUG
    println!("focal distance: {}", focal_distance);
    ////////

    // make multiplier point
    // this point will contain θ for each axis to rotate around
    let mult: Point = Point (prompt_int("X rot: "), prompt_int("Y rot: "), prompt_int("Z rot: "));

    // our original vector array will stay the same because projecting it returns a new line2
    // loop through it until we do a keyboard escape
    loop {

        // project and draw, then rotate each line
        for line in vec {
            // project & draw
            line.project(focal_distance).draw();

            // rotate
            line.rotate(mult);
        }

    }
    
}
