
use nom::IResult;
use std::ops::Add;
use std::ops::Sub;
use ndarray::ArrayBase;
use ndarray::Dim;
use ndarray::OwnedRepr;
use ndarray::Array2;
use std::cmp;


#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum NextStep {
    NextPoint(Point),
    Stuck,
    Falling,
}

impl Add<&Point> for & Point {
    type Output = Point;

    fn add(self, other: & Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Point> for & Point {
    type Output = Point;

    fn sub(self, other: & Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}




fn direction(p: &Point) -> Point {
    let x = if p.x >= 1 {
       1 
    } else if p.x <= -1 {
        -1
    } else {
        0
    };
    let y = if p.y >= 1 {
        1 
     } else if p.y <= -1 {
         -1
     } else {
         0
     };
     Point{
        x : x,
        y : y
     }
}

fn get_range(a: &Point, b: &Point) -> Vec::<Point>{
    let dir = direction(&(b - a));
    let mut offset = Point {x:0, y:0};
    let mut l = Vec::<Point>::new();
    while(a + &offset != *b) {
        l.push(a + &offset);
        offset = &offset + &dir; 
    }
    l.push(a + &offset);
    l
}

fn parse_cave(input: &str) -> ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>> {
    let mut walls: Vec::<Vec::<Point>> = Vec::new();
    let mut corners: Vec::<Point>;
    let mut iter;
    let mut x: i32;
    let mut y: i32;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for line in input.lines(){
        corners = Vec::new();
        for (i, pair) in line.split(" -> ").enumerate() {
            iter = pair.split(",");
            x = iter.next().unwrap().parse::<i32>().unwrap();
            y = iter.next().unwrap().parse::<i32>().unwrap();
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            corners.push(Point {
                x: x,
                y: y,
            });          
        }
        walls.push(corners);
    }

    let mut cave = Array2::<i32>::zeros((max_y as usize+3, max_x as usize +  2*(max_y as usize+1)));
    
    let mut prev_corner = &Point{x:0, y:0};
    for wall in walls.iter() {
        for (i, corner) in wall.iter().enumerate() {
            if i == 0  {
                prev_corner = corner;
                continue;
            }
            for point in get_range(corner, prev_corner).iter(){
                cave[[point.y as usize, point.x as usize]] = 1;
            };
            prev_corner = corner;
        }
    }
    for i in 0..cave.ncols() {
        let r:usize = cave.nrows()-1;
        cave[[r, i]] = 1;
    }

    cave
}

fn get_next_pos(cave: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>, current: &Point) -> NextStep {
    let mut next_point = *current;
    next_point.y = next_point.y + 1;
    match cave.get((next_point.y as usize, next_point.x as usize)) {
        None => {
            println!("Falling 2 {:?}", next_point);
            return NextStep::Falling;
        },
        Some(i) => {
            if *i == 0 as i32{
                return NextStep::NextPoint(next_point);
            };
        },
    };
    next_point.x = next_point.x - 1;
    match cave.get((next_point.y as usize, next_point.x as usize)) {
        None => {
            println!("Falling 2 {:?}", next_point);
            return NextStep::Falling;
        },
        Some(i) => {
            if *i == 0 as i32{
                return NextStep::NextPoint(next_point);
            };
        },
    };

    next_point.x = next_point.x + 2;
    match cave.get((next_point.y as usize, next_point.x as usize)) {
        None => {
            println!("Falling 3 {:?}", next_point);
            return NextStep::Falling;
        },
        Some(i) => {
            if *i == 0 as i32{
                return NextStep::NextPoint(next_point);
            };
        },
    };


    return NextStep::Stuck;
}


fn drop_stone(cave: &ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>, source: &Point) -> Option<Point> {
    match cave.get((source.y as usize, source.x as usize)) {
        Some(i) => if *i == 1 {
            return None;
        },
        None => panic!("Source Is outside of matrx!"), 
    }
    let mut current = *source;
    loop {
        match get_next_pos(cave, &current) {
            NextStep::NextPoint(p) => current = p,
            NextStep::Stuck => return Some(current),
            NextStep::Falling => panic!("Falling..."),  
        }  
    }
}



pub fn main(input: &str) -> String {
    let mut cave = parse_cave(input);
    let source = Point{x:500, y:0};
    let mut stones = 0;
    println!("Shape {:?}", cave.shape());
    loop {
        match drop_stone(&cave, &source) {
            Some(p) => cave[[p.y as usize, p.x as usize]] = 1,
            None => break,
        }
        stones += 1;
    }

    "Done".to_string()
}
