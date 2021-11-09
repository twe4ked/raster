use super::Vec3;
use std::io::{self, Write};

// https://en.wikipedia.org/wiki/Netpbm_format#PPM_example

pub fn write_header(output: &mut dyn Write, width: usize, height: usize) -> io::Result<()> {
    writeln!(output, "P3\n{} {}\n255", width, height)
}

pub fn write_color(output: &mut dyn Write, color: &Vec3) -> io::Result<()> {
    let r = color[0] as usize;
    let g = color[1] as usize;
    let b = color[2] as usize;
    writeln!(output, "{} {} {}", r, g, b)
}
