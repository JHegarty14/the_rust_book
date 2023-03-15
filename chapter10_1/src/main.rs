use std::cmp::PartialOrd;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct NewPoint<X1, Y1> {
    x: X1,
    y: Y1
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> NewPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: NewPoint<X2, Y2>) -> (NewPoint<X1, Y2>, NewPoint<X2, Y1>) {
        let p1 = NewPoint {
            x: self.x,
            y: other.y,
        };
        let p2 = NewPoint {
            x: other.x,
            y: self.y,
        };
        (p1, p2)
    }
}

fn main() {
    // let num_list = vec![34, 50, 25, 100, 65];

    // let mut largest = &num_list[0];

    // for num in &num_list {
    //     if num > largest {
    //         largest = num;
    //     }
    // }

    // println!("The largest number is: {largest}");

    // let num_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // for num in &num_list {
    //     if num > largest {
    //         largest = num;
    //     }
    // }

    // println!("The largest number is: {largest}");

    let num_list = vec![34, 50, 25, 100, 65];

    let mut largest_num = largest_i32(&num_list);

    println!("The largest number is: {largest_num}");

    let num_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    largest_num = largest_i32(&num_list);

    println!("The largest number is: {largest_num}");

    let char_list = vec!['y', 'x', 'q', 'a', 'b', 'i', 'p', 'h', 'j', 'z'];

    let found = largest_char(&char_list);

    println!("The largest char is: {found}");

    let test_one = largest(&num_list);

    println!("test one: {test_one}");

    let test_two = largest(&char_list);

    println!("test two: {test_two}");

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 3.3, y: 4.4 };

    println!("int_point: {:#?}", int_point);
    println!("float_point: {:#?}", float_point);

    println!("int_point x: {:#?}", int_point.x());
    println!("float_point x: {:#?}", float_point.x());
    
    // won't compile: cannot mix types with generics
    // let wont_work = Point { x: 5, y: 4.4 };

    let f32_point: Point<f32> = Point { x: 1.1, y: 2.2 };
    println!("Distance from origin: {}", f32_point.distance_from_origin());

    // won't compile, distance_from_origin exists on f32-specific struct
    // println!("int Distance from origin: {}", int_point.distance_from_origin());

    let p1 = NewPoint { x: 5, y: 10.3 };
    let p2 = NewPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("Mixed up: {:#?}, {:#?}", p3.0, p3.1);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for c in list {
        if c > largest {
            largest = c;
        }
    }

    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for v in list {
        if v > largest {
            largest = v;
        }
    }

    largest
}