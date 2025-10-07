use std::cmp;

fn fun(x:f32) -> f32 {
    x.sin() * 10.
}

fn run_func(v: Vec<f32>, callback: fn(f32) -> f32) -> Vec<f32> {
    v.into_iter().map(|x| callback(x)).collect()
}

fn graph(dots: Vec<f32>) {
    let width = dots.len();
    let mut height_top: i32 = 0;
    let mut height_bottom: i32 = 0;
    let height;

    for n in &dots {
        if *n == 0. {
            continue;
        }

        let sign = n / n;

        // positive
        if sign == 1. {
            height_top = cmp::max(height_top, n.round() as i32);
        // negative
        } else {
            height_bottom = cmp::min(height_bottom, n.round() as i32);
        }
    }

    height = height_top + height_bottom * -1;

    println!("{} and {}", width, height);

    for y in 1..=height {
        for x in 1..width {
            if y == dots[x].round() as i32 {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {

    let mut nums: Vec<f32> = Vec::new();

    let mut n: f32 = 0.;

    for _ in 0..32 {
        nums.push(n.clone());
        n += 0.1;
    }

    let nums_after: Vec<f32> = run_func(nums, fun);

    for n in &nums_after {
        println!("{} ", n);
    }

    graph(nums_after);
}
