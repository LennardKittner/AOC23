use itertools::Itertools;
use gomez::nalgebra as na;
use gomez::{Domain, Problem, SolverDriver, System};
use na::{Dyn, IsContiguous};

#[derive(Clone)]
struct Hail {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

// y = mx + b
#[derive(Debug)]
struct Line {
    m: f64,
    b: f64
}

fn hail_to_line(hail: &Hail) -> Line {
    let m = hail.vy / hail.vx;
    Line { m , b: hail.py - m * hail.px }
}

fn find_intersection(line1: &Line, line2: &Line) -> Option<(f64, f64)> {
    if line1.m == line2.m {
        return None;
    }
    let x = (line2.b - line1.b) / (line1.m - line2.m);
    let y = line1.m * x + line1.b;
    Some((x, y))
}

fn in_the_past(p: &(f64, f64), hail: &Hail) -> bool {
    let vx = p.0 - hail.px;
    let vy = p.1 - hail.py;
    let m1 = vx / hail.vx;
    let m2 = vy / hail.vy;
    m1 < 0f64 && m2 < 0f64
}

pub fn exec_day24_part1(input: &str) -> String {
    let hails = parse(input);
    let lines = hails.iter().map(hail_to_line).collect_vec();

    let limit = 200000000000000f64..=400000000000000f64;
    let mut result = 0;
    for (i, l1) in lines.iter().enumerate() {
        for j in i..lines.len() {
            if let Some(a) = find_intersection(l1, &lines[j]) {
                if limit.contains(&a.0) && limit.contains(&a.1) && !in_the_past(&a, &hails[i]) && !in_the_past(&a, &hails[j]) {
                    result += 1;
                }
            }
        }
    }

    result.to_string()
}

fn parse(input: &str) -> Vec<Hail> {
    let hails = input.lines().map(|line| {
        let values = line.split(" @ ").map(|v| v.split(", ").map(|val| val.trim().parse::<i64>().unwrap() as f64).collect_vec()).collect_vec();
        Hail {
            px: values[0][0],
            py: values[0][1],
            pz: values[0][2],
            vx: values[1][0],
            vy: values[1][1],
            vz: values[1][2],
        }
    }).collect_vec();
    hails
}

struct Equations {
    h1: Hail,
    h2: Hail,
    h3: Hail,
}

impl Problem for Equations {
    type Field = f64;

    fn domain(&self) -> Domain<Self::Field> {
        Domain::unconstrained(9)
    }
}

impl System for Equations {
    // Evaluation of the system (computing the residuals).
    fn eval<Sx, Srx>(
        &self,
        x: &na::Vector<Self::Field, Dyn, Sx>,
        rx: &mut na::Vector<Self::Field, Dyn, Srx>,
    ) where
        Sx: na::storage::Storage<Self::Field, Dyn> + IsContiguous,
        Srx: na::storage::StorageMut<Self::Field, Dyn>,
    {
        rx[0] = 1.0*x[0] + 0.0*x[1] + 0.0*x[2] + x[6]*x[3] + 0.0*x[4] + 0.0*x[5] - (self.h1.px + x[6]*self.h1.vx);
        rx[1] = 0.0*x[0] + 1.0*x[1] + 0.0*x[2] + 0.0*x[3] + x[6]*x[4] + 0.0*x[5] - (self.h1.py + x[6]*self.h1.vy);
        rx[2] = 0.0*x[0] + 0.0*x[1] + 1.0*x[2] + 0.0*x[3] + 0.0*x[4] + x[6]*x[5] - (self.h1.pz + x[6]*self.h1.vz);
        rx[3] = 1.0*x[0] + 0.0*x[1] + 0.0*x[2] + x[7]*x[3] + 0.0*x[4] + 0.0*x[5] - (self.h2.px + x[7]*self.h2.vx);
        rx[4] = 0.0*x[0] + 1.0*x[1] + 0.0*x[2] + 0.0*x[3] + x[7]*x[4] + 0.0*x[5] - (self.h2.py + x[7]*self.h2.vy);
        rx[5] = 0.0*x[0] + 0.0*x[1] + 1.0*x[2] + 0.0*x[3] + 0.0*x[4] + x[7]*x[5] - (self.h2.pz + x[7]*self.h2.vz);
        rx[6] = 1.0*x[0] + 0.0*x[1] + 0.0*x[2] + x[8]*x[3] + 0.0*x[4] + 0.0*x[5] - (self.h3.px + x[8]*self.h3.vx);
        rx[7] = 0.0*x[0] + 1.0*x[1] + 0.0*x[2] + 0.0*x[3] + x[8]*x[4] + 0.0*x[5] - (self.h3.py + x[8]*self.h3.vy);
        rx[8] = 0.0*x[0] + 0.0*x[1] + 1.0*x[2] + 0.0*x[3] + 0.0*x[4] + x[8]*x[5] - (self.h3.pz + x[8]*self.h3.vz);
    }
}

// 1*x1 + 0*x2 + 0*x3 + u*x4 + 0*x5 + 0*x6 = 260346828765750 + u*64
// 0*x1 + 1*x2 + 0*x3 + 0*x4 + u*x5 + 0*x6 = 357833641339849 + u*(-114)
// 0*x1 + 0*x2 + 1*x3 + 0*x4 + 0*x5 + u*x6 = 229809969824403 + u*106
// 1*x1 + 0*x2 + 0*x3 + v*x4 + 0*x5 + 0*x6 = 340220726383465 + v*(-79)
// 0*x1 + 1*x2 + 0*x3 + 0*x4 + v*x5 + 0*x6 = 393110064924024 + v*(-61)
// 0*x1 + 0*x2 + 1*x3 + 0*x4 + 0*x5 + v*x6 = 226146987100003 + v*158
// 1*x1 + 0*x2 + 0*x3 + t*x4 + 0*x5 + 0*x6 = 11361697274707 + t*328
// 0*x1 + 1*x2 + 0*x3 + 0*x4 + t*x5 + 0*x6 = 101596061919750 + t*162
// 0*x1 + 0*x2 + 1*x3 + 0*x4 + 0*x5 + t*x6 = 46099495948720 + t*333

pub fn exec_day24_part2(input: &str) -> String {
    // https://matrixcalc.org/slu.html#solve-using-Gaussian-elimination%28%7B%7B1,0,0,u,0,0,0,0,0,260346828765750+u*64%7D,%7B0,1,0,0,u,0,0,0,0,357833641339849+u*%28-114%29%7D,%7B0,0,1,0,0,u,0,0,0,229809969824403+u*106%7D,%7B1,0,0,v,0,0,0,0,0,340220726383465+v*%28-79%29%7D,%7B0,1,0,0,v,0,0,0,0,393110064924024+v*%28-61%29%7D,%7B0,0,1,0,0,v,0,0,0,226146987100003+v*158%7D,%7B1,0,0,t,0,0,0,0,0,11361697274707+t*328%7D,%7B0,1,0,0,t,0,0,0,0,101596061919750+t*162%7D,%7B0,0,1,0,0,t,0,0,0,46099495948720+t*333%7D%7D%29
    // The following code does not produce the same answer. This could be because I don't know how to use Gomez and made a mistake, or there could be multiple valid solutions.
    let hails = parse(input);

    let r = Equations {
        h1: hails[0].clone(),
        h2: hails[1].clone(),
        h3: hails[2].clone(),
    };
    let mut solver = SolverDriver::builder(&r)
        .with_initial(vec![242720827369528.0, 344525619959965.0, 437880958119624.0, 100.0, 100.0, 100.0 , 780243378654.0, 215244678825.0, 516434301805.0])
        .build();

    let (x, _) = solver
        .find(|state| state.norm() <= 0.0 || state.iter() >= 100)
        .expect("solver error");

    (x[0]+x[1]+x[2]).to_string()
}
