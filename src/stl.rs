use crate::linear_algebra as la;
use crate::primitive;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::path;

fn read_vector(f: &mut fs::File) -> Result<la::Vector, io::Error> {
    let mut buffer = [0; 4];

    let _ = f.read(&mut buffer[..])?;
    let x = f32::from_le_bytes(buffer) as f64;
    let _ = f.read(&mut buffer[..])?;
    let y = f32::from_le_bytes(buffer) as f64;
    let _ = f.read(&mut buffer[..])?;
    let z = f32::from_le_bytes(buffer) as f64;

    Ok(la::Vector::new(x, y, z))
}

pub fn load_stl(file: &path::Path) -> Result<Vec<primitive::Triangle>, io::Error> {
    let mut f = fs::File::open(file)?;
    let mut buffer = [0; 4];

    let _ = f.read(&mut buffer[..])?;
    let facets = u32::from_le_bytes(buffer);

    let mut triangles = Vec::new();

    for _ in 0..facets {
        let _normal = read_vector(&mut f)?;
        let v1 = read_vector(&mut f)?;
        let v2 = read_vector(&mut f)?;
        let v3 = read_vector(&mut f)?;

        f.seek(io::SeekFrom::Current(2))?;

        let triangle = primitive::Triangle::new(v1, v2, v3, 0, None, None);
        triangles.push(triangle);
    }

    Ok(triangles)
}
