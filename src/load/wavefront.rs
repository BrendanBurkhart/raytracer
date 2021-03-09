use obj;
use std::collections::HashMap;
use std::io;
use std::path;

use super::scene;
use super::linear;

fn to_vector(vertex: &[f32; 3]) -> linear::Vector {
    linear::Vector::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64)
}

fn to_uv(tvertex: &[f32; 2]) -> scene::lighting::UV {
    scene::lighting::UV::new(tvertex[0] as f64, tvertex[1] as f64)
}

fn convert_color(color: Option<[f32; 3]>) -> scene::lighting::Color {
    match color {
        None => scene::lighting::Color::black(),
        Some(color) => scene::lighting::Color::new(color[0] as f64, color[1] as f64, color[2] as f64),
    }
}

fn get_dir(file: &path::Path) -> path::PathBuf {
    let mut path = path::PathBuf::from(file);
    path.pop();

    path
}

fn convert_material(material: &obj::Material, base_path: &path::PathBuf) ->scene:: lighting::Material {
    let specular = convert_color(material.ks);
    let diffuse = convert_color(material.kd);
    let ambient = convert_color(material.ka);

    let alpha = match material.ns {
        None => 0.0,
        Some(alpha) => alpha as f64,
    };

    let texture = match &material.map_kd {
        None => scene::lighting::Texture::new(Vec::new(), 0, 0),
        Some(path) => {
            let mut full_path = path::PathBuf::from(base_path);
            full_path.push(path);
            let file = image::open(full_path).expect("Image texture must load correctly");
            let image = file.to_rgb();
            let data = image.to_vec();
            let width = image.width() as usize;
            let height = image.height() as usize;

            scene::lighting::Texture::new(data, width, height)
        }
    };

    scene::lighting::Material::new(
        specular,
        diffuse,
        ambient,
        alpha,
        0.1,
        texture,
    )
}

fn tessellate(
    polygon: &Vec<obj::IndexTuple>,
    object: &obj::Obj<Vec<obj::IndexTuple>>,
    material_index: usize,
    mesh: &mut Vec<scene::primitive::Triangle>,
) {
    let anchor = polygon[0];
    let a = to_vector(&object.position[anchor.0]);

    let has_texture = match anchor.1 {
        None => false,
        Some(_) => true,
    };

    let has_normals = match anchor.2 {
        None => false,
        Some(_) => true,
    };

    for others in polygon[1..].windows(2) {
        let b = to_vector(&object.position[others[0].0]);
        let c = to_vector(&object.position[others[1].0]);

        let texture_map = match has_texture {
            false => None,
            true => {
                let x = to_uv(&object.texture[anchor.1.unwrap()]);
                let y = to_uv(&object.texture[others[0].1.unwrap()]);
                let z = to_uv(&object.texture[others[1].1.unwrap()]);
                Some((x, y, z))
            }
        };

        let normal_map = match has_normals {
            false => None,
            true => {
                let x = to_vector(&object.normal[anchor.2.unwrap()]);
                let y = to_vector(&object.normal[others[0].2.unwrap()]);
                let z = to_vector(&object.normal[others[1].2.unwrap()]);
                Some((x, y, z))
            }
        };

        let t = scene::primitive::Triangle::new(a, b, c, material_index, texture_map, normal_map);
        mesh.push(t);
    }
}

pub fn load_obj(
    file: &path::Path,
) -> Result<(Vec<scene::lighting::Material>, Vec<scene::primitive::Triangle>), io::Error> {
    let mut object: obj::Obj<obj::SimplePolygon> = obj::Obj::load(file)?;
    let mtls = object.load_mtls();

    if let Err(errors) = mtls {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Couldn't load mtl {}", errors[0].0),
        ));
    }

    let mut mesh = Vec::new();

    let mut materials_index = HashMap::new();
    let mut materials = Vec::new();
    let mut current_material = 1;

    materials.push(scene::lighting::Material::new(
        scene::lighting::Color::black(),
        scene::lighting::Color::black(),
        scene::lighting::Color::black(),
        0.0,
        0.0,
        scene::lighting::Texture::new(Vec::new(), 0, 0),
    ));

    materials_index.insert("none", 0);

    let resource_dir = get_dir(file);

    for o in &object.objects {
        for g in &o.groups {
            for polygon in &g.polys {
                let mut material_name = "none";
                if let Some(material_ref) = &g.material {
                    material_name = &material_ref.name;
                    if !(materials_index.contains_key(material_name)) {
                        materials.push(convert_material(material_ref, &resource_dir));
                        materials_index.insert(material_name, current_material);
                        current_material = current_material + 1;
                    }
                }

                let index = materials_index.get(material_name).unwrap_or(&0);
                let index = (*index) as usize;

                tessellate(&polygon, &object, index, &mut mesh);
            }
        }
    }

    Ok((materials, mesh))
}
