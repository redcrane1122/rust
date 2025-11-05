trait Fall {
    fn hit_ground(&self);
}

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Vase;

impl Fall for Vase {
    fn hit_ground(&self) {
        println!("the vas broke!");
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("the cat casually walked away");
    }
}

struct Square {
    side: i32
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side_a + self.side_b + self.side_c
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self { side_a: 0, side_b: 0, side_c: 0 }
    }
}

fn fall(thing: impl Fall) {
    thing.hit_ground();
}

fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("perimeter = {:?}", perimeter);
}

fn main() {
    fall(Vase {});
    fall(Cat {});

    let square = Square { side: 5};
    let triangle = Triangle {
        side_a: 3,
        side_b: 3,
        side_c: 5,
    };

    print_perimeter(square);
    print_perimeter(triangle);
}