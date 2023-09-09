#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // version con tuplas

    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        fn_dimensions(rect)
    );

    // version struct
    let rect_struct = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect_struct)
    );
    println!("{:#?}", rect_struct);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn fn_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
