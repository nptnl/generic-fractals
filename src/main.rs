use basemath::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static HEX_GS: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f',];

fn main() {
    pspace(Comp::new(0.0, 0.0), 128, 2.0, 8);
}

fn ispace(c: Comp, size: u32, planesize: f64, iterate: usize) {

    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).expect("cannot write header");
    
    let formula = |z: Comp| exp(z*z) + ln(z) - z*z + c;

    let step: f64 = planesize / size as f64;
    let mut crnt: Comp = Comp::new(-planesize + 0.5*step, planesize - 0.5*step);
    let mut counter: usize;
    let mut valstring: String;
    let mut z: Comp;

    for _ in 0..size*2 {
        valstring = String::new();
        crnt.r = -planesize + 0.5*step;
        for _ in 0..size*2 {
            counter = 0;
            z = crnt;
            loop {
                if counter == iterate { counter = 0; break }
                if z.r*z.r + z.i*z.i > planesize * planesize { break }
                z = formula(z);
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

fn pspace(seed: Comp, size: u32, planesize: f64, iterate: usize) {

    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).expect("cannot write header");

    let formula = | z: Comp, c: Comp | z.inv() + c;

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
