#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // Calculate the area of a given rectangle

    // ex with variables (bad because vars are not grouped, need to pass two args to func)
    let width1 = 30;
    let height1 = 20;
    println!(
        "The area of rectangle1 ({}, {}) is {}",
        width1,
        height1,
        area_vars(width1, height1)
    );

    // ex with touples (improved readability, but still bad because touple elements are not named)
    let rect1: (u32, u32) = (30, 40);
    println!(
        "The area of rectangle1 ({}, {}) is {}",
        rect1.0,
        rect1.1,
        area_touple(rect1)
    );

    // ex with struct (best readability, only thing left is to reduce width and height print vars to one)
    let rect1 = Rectangle {
        width: 32,
        height: 64,
    };
    println!(
        "The area of rectangle1 ({}, {}) is {}",
        rect1.width,
        rect1.height,
        area_struct(&rect1)
    );

    // to reduce vars for println!, Display or Debug trait is needed. 
    //To implement Debug write above struct #[derive(Debug)]

    // to display in Debug mode (useful for devs) use {:?} and {:#?} in println! or dbg!(rect1)
    // {:#?} increases readability for larger structs

    //Note: Calling the dbg! macro prints to the standard error console stream (stderr),
    //as opposed to println! which prints to the standard output console stream (stdout).

    println!(
        "Debug trait implemented here is the output of rect1: {:?}",
        rect1
    );
    //dbg!(&rect1);
}

fn area_vars(i: u32, p: u32) -> u32 {
    i * p
}
fn area_touple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
