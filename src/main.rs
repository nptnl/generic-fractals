use basemath::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static HEX_GS: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f',];

fn main() {
    ispace(Comp::nre(-0.5), 512, 16);
}

fn ispace(c: Comp, size: u32, iterate: usize) {

    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes());
    
    let formula = |z: Comp| z * z + c;
    let step: f64 = 2.0 / size as f64;
    let mut crnt: Comp = Comp::new(-2.0 + 0.5*step, 2.0 - 0.5*step);
    let mut counter: usize = 0;
    let mut valstring;
    let mut z;

    for _ in 0..size*2 {
        valstring = String::new();
        crnt.r = -2.0 + 0.5*step;
        for _ in 0..size*2 {
            counter = 0;
            z = crnt;
            loop {
                if counter == iterate { counter = 0; break }
                if z.r*z.r + z.i*z.i > 4.0 { break }
                z = formula(z);
                counter += 1;
            }
            valstring = format!("{valstring}{}", HEX_GS[counter * 16 / iterate]);
            crnt.r += step;
        }
        valstring += "\n";
        file.write_all(valstring.as_bytes());
        crnt.i -= step;
    }

}