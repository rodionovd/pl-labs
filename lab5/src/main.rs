use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::f32;
use std::env;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point
}

#[derive(Debug)]
struct Ray {
    start: Point,
    other: Point
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args[0].as_ref() {
        "./main" => Path::new("../input.txt"),
        _ => {
            if args.len() < 2 {
                panic!("Please specify an input file as an argument for this program.");
            }
            Path::new(&args[1])
        }
    };

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file
    };
    let reader = BufReader::new(&file);
    let mut inputs = reader.lines();
    
    let header = inputs.nth(0).unwrap().ok().unwrap();
    let ray = Ray::new(header);
    let answer = inputs.filter_map(|l| Segment::new(l.unwrap())).fold(false, |acc, s| acc || ray.intersects(s));	
    print!("{}", if answer == true {
        "Есть пересечение!\n"
    } else {
        "Пересечений нет :/\n"
    });
}

impl Ray {
    fn new(line: String) -> Ray {
        let raw = line_input_to_points(line);
        Ray { start: Point{x: raw[0], y: raw[1]}, other: Point{x: raw[2], y: raw[3]} }
    }
}

impl Segment {
    fn new(line: String) -> Option<Segment> {
        if line.starts_with("//") {
            None
        } else  {
            let raw = line_input_to_points(line);
            Some(Segment { start: Point{x: raw[0], y: raw[1]}, end: Point{x: raw[2], y: raw[3]} }	)
        }
    }
}

pub fn line_input_to_points(line: String) -> Vec<f32> {
    line.split(" ").flat_map(|r| r.split(",")).filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>()
}

pub fn determinant2(a00: f32, a01: f32, a10: f32, a11: f32) -> f32 {
    return (a00 * a11) - (a01 * a10);
}

impl Ray {
    fn intersects(&self, seg: Segment) -> bool {
        //
        // Луч задаётся следующей системой уравнений:
        // | x = P1.x + t * (P2.x - P1.x)
        // | y = P1.y + t * (P2.y - P1.y)
        // Отрезок:
        // | x = A.x * s + B.x * (1-s) = s*(A.x) + B.x - s*(B.x) = s*(A.x - B.x) + B.x
        // | y = A.y * s + B.y * (1-s) = s*(A.y) + B.y - s*(B.y) = s*(A.y - B.y) + B.y
        //
        // Для пересечения необходимо наличие решение у системы вида
        // | P1.x + t * (P2.x - P1.x) - s*(A.x - B.x) - B.x = 0
        // | P1.y + t * (P2.y - P1.y) - s*(A.y - B.y) - B.y = 0
        // где t >= 0 и 0 <= s <= 1 - неизвестные
        //
        // Решаем данную систему методом Крамера. Для начала преобразуем систему к канонической форме
        // | t * (P2.x - P1.x) - s * (A.x - B.x) = B.x - P1.x
        // | t * (P2.y - P1.y) - s * (A.y - B.y) = B.y - P1.y
        //
        // Составим матрицу M данной системы уравнений:
        // M = | P2.x-P1.x  -A.x+B.x |
        //     | P2.y-P1.y  -A.y+B.y |
        let a00 = self.other.x - self.start.x;
        let a01 = -seg.start.x + seg.end.x;
        let a10 = self.other.y - self.start.y;
        let a11 = -seg.start.y + seg.end.y;
        // вектор b свободных членов:
        // b = | B.x - P1.x |
        //     | B.y - P1.y |
        let b0 = seg.end.x - self.start.x;
        let b1 = seg.end.y - self.start.y;
        // Обозначим за D определитель матрицы M = |M|.
        let d = determinant2(a00, a01, a10, a11);
        // >>> Если определитель системы равен нолю, значит она не имеет решений. В нашем случае луч не
        // пересекает отрезок.
        if d.abs() < f32::EPSILON {
            return false
        }
        // За D1 обозначим определитель матрицы M, у которой первый столбец заменён на вектор b:
        // M1 = | B.x - P1.x  -A.x+B.x |
        //      | B.y - P1.y  -A.y+B.y | 
        // D1 = |M1|
        let d1 = determinant2(b0, a01, b1, a11);
        // Аналогично с D2, только теперь заменяем второй столбец матрицы M на вектор b:
        // M2 = | P2.x-P1.x  B.x - P1.x |
        //      | P2.y-P1.y  B.y - P1.y |
        // D2 = |M2|
        let d2 = determinant2(a00, b0, a10, b1);
        // Вычислим t и s:
        // | t = D1 / D
        // | s = D2 / D
        let t = d1 / d;
        let s = d2 / d;
        // Последним шагом проверим выполнение условий t >= 0 и 0 <= s <= 1:
        // если они выполняются, значит заданные луч и отрезок пересекаются
        (t >= 0.0) && (0.0 <= s) && (s <= 1.0)
    }
}
