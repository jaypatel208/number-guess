#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

impl Rectangle {
    fn area_from_struct(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the given rectangle is: {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (20, 40);
    println!("The area of rect1 here is: {}", area_from_tuple(rect1));

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(39 * scale),
        height: 12,
    };

    println!("The area of rect2 is: {}", rect2.area_from_struct());

    println!("rect2 is {:#?}", rect2);
    dbg!(&rect1);

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect2))
}
