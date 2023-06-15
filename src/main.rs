use basemath::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;

static HEX_GS: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f',];

fn main() {
    let par: Comp = Comp { r: 0.0, i: 0.0 };
    let topleft: Comp = Comp { r: -1.5, i: 1.4 };
    let bottomright: Comp = Comp { r: 3.0, i: -0.4 };
    let bound: f64 = 2.0;
    let bound = bound * bound;
    let width: u32 = 1024;
    let height: u32 = 1024;
    let iterate: usize = 16;
    let num: usize = 1;
    let threads: u32 = 8;

    multi_i(par, topleft, bottomright, bound, width, height, iterate, threads);
}

fn multi_i(
    par: Comp,
    topleft: Comp,
    bottomright: Comp,
    bound: f64,
    width: u32,
    height: u32,
    iterate: usize,
    threads: u32,
) {
    let separation: f64 = (topleft.i - bottomright.i) * 0.125;
    let mut loc_tl: Comp = topleft;
    let mut loc_br: Comp = Comp { r: bottomright.r, i: topleft.i - separation};
    for parallel in 1..threads+1 {
        ispace(par, loc_tl, loc_br, bound, width, height / threads, iterate, parallel as usize);
        loc_tl.i -= separation;
        loc_br.i -= separation;
    }
}

fn combine_npxl(count: usize, width: u32, height: u32) {
    let path = Path::new("./plots/cur.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", width, height) + "16 1\n";
    file.write_all(first.as_bytes()).expect("cannot write header");

    let mut name: String = String::new();
    for num in 1..count+1 {
        name = format!("./plots/build/{num}.npxlb");
    }
}

#[allow(dead_code)]
fn ispace(
    par: Comp,
    topleft: Comp,
    bottomright: Comp,
    bound: f64,
    width: u32,
    height: u32,
    iterate: usize,
    num: usize
) {

    let name: String = format!("./plots/build/{num}.npxlb");
    let path = Path::new(name.as_str());
    let mut file = File::create(&path).unwrap();
    
    let formula = |z: Comp, c: Comp| z*z + c;

    let rstep: f64 = (bottomright.r - topleft.r) / width as f64;
    let istep: f64 = (topleft.i - bottomright.i) / height as f64;
    let mut crnt: Comp = topleft + Comp::new(0.5*rstep, -0.5*istep);
    let mut counter: usize;
    let mut valstring: String;
    let mut z: Comp;

    for _ in 0..height {
        valstring = String::new();
        crnt.r = topleft.r + 0.5*rstep;
        for _ in 0..width {
            counter = 0;
            z = crnt;
            loop {
                if counter == iterate { counter = 0; break }
                if z.r*z.r + z.i*z.i > bound * bound { break }
                z = formula(z, par);
                counter += 1;
            }
            valstring = format!("{valstring}{}", HEX_GS[counter * 16 / iterate]);
            crnt.r += rstep;
        }
        valstring += "\n";
        file.write_all(valstring.as_bytes()).expect("cannot write line");
        crnt.i -= istep;
    }

}

#[allow(dead_code)]
fn pspace(seed: Comp, size: u32, planesize: f64, iterate: usize) {

    let path = Path::new("./plots/pcurrent.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).expect("cannot write header");

    let formula = |z: Comp, c: Comp| z*z + c;

    let step: f64 = planesize / size as f64;
    let mut crnt: Comp = Comp::new(-planesize + 0.5*step, planesize - 0.5*step);
    let mut counter: usize;
    let mut valstring: String;
    let mut z: Comp;
    let mut c: Comp;

    for _ in 0..size*2 {
        valstring = String::new();
        crnt.r = -planesize + 0.5*step;
        for _ in 0..size*2 {
            counter = 0;
            z = seed;
            c = crnt;
            loop {
                if counter == iterate { counter = 0; break }
                if z.r*z.r + z.i*z.i > planesize * planesize { break }
                z = formula(z, c);
                counter += 1;
            }
            valstring = format!("{valstring}{}", HEX_GS[counter * 16 / iterate]);
            crnt.r += step;
        }
        valstring += "\n";
        file.write_all(valstring.as_bytes()).expect("cannot write line");
        crnt.i -= step;
    }

}
