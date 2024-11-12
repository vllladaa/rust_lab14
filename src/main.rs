#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn rectangle_area(rect: &Rectangle) -> i32 {
    let width = (rect.b.x - rect.a.x).abs();
    let height = (rect.a.y - rect.b.y).abs();
    width * height
}

fn intersection_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = (r1.b.x.min(r2.b.x) - r1.a.x.max(r2.a.x)).max(0);
    let y_overlap = (r1.a.y.min(r2.a.y) - r1.b.y.max(r2.b.y)).max(0);
    x_overlap * y_overlap
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let n = rectangles.len();

    for rect in rectangles {
        total_area += rectangle_area(rect);
    }

    for i in 0..n {
        for j in (i + 1)..n {
            total_area -= intersection_area(&rectangles[i], &rectangles[j]);
        }
    }

    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Test passed! Calculated occupied area: {}", occupied);
}

fn main() {
    area_occupied_test();
}