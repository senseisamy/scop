use super::{Object, Vertexxx};
use crate::math::Vec3;
use std::{collections::HashMap, error::Error, usize};

impl Object {
    pub fn parse(object: &str) -> Result<Self, Box<dyn Error>> {
        let mut v: Vec<[[f32; 3]; 2]> = vec![[[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]];
        let mut vt: Vec<[f32; 2]> = vec![[0.0, 0.0]];
        let mut vn: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]];

        let mut unique_vertices: HashMap<Vertexxx, u32> = HashMap::new();
        let mut obj = Object {
            vertex: Vec::from([Vertexxx::default()]),
            indice: Vec::new(),
            size: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        for (line_number, line) in object.lines().enumerate() {
            let line_number = line_number + 1;
            let line: Vec<&str> = line
                .split_ascii_whitespace()
                .take_while(|x| !x.contains("#"))
                .collect();
            if line.len() < 2 {
                continue;
            }
            match line[0] {
                "v" => {
                    if line.len() == 4 {
                        v.push([
                            [line[1].parse()?, line[2].parse()?, line[3].parse()?],
                            [1.0, 1.0, 1.0],
                        ]);
                    } else if line.len() == 7 {
                        let mut color: [f32; 3] =
                            [line[4].parse()?, line[5].parse()?, line[6].parse()?];
                        for c in color.iter_mut() {
                            if *c > 1.0 {
                                *c = *c / 255.0;
                            }
                        }
                        v.push([
                            [line[1].parse()?, line[2].parse()?, line[3].parse()?],
                            color,
                        ]);
                    } else {
                        return Err(
                            format!("obj parsing error: line {line_number}: expected (x, y, z [, r, g, b]) format").into()
                        );
                    }
                }
                "vt" => {
                    if line.len() < 3 || line.len() > 4 {
                        return Err(format!(
                            "obj parsing error: line {line_number}: expected (u, v, [w]) format"
                        )
                        .into());
                    }
                    let tmp: [f32; 2] = [line[1].parse()?, line[2].parse()?];
                    vt.push([tmp[0], 1.0 - tmp[1]]);
                }
                "vn" => {
                    if line.len() != 4 {
                        return Err(format!(
                            "obj parsing error: line {line_number}: expected (x, y, z) format"
                        )
                        .into());
                    }
                    vn.push([line[1].parse()?, line[2].parse()?, line[3].parse()?]);
                }
                "f" => {
                    if line.len() == 4 {
                        let (v1, has_normal, _has_texture) = parse_face_el(line[1], &v, &vt, &vn)?;
                        let (v2, _, _) = parse_face_el(line[2], &v, &vt, &vn)?;
                        let (v3, _, _) = parse_face_el(line[3], &v, &vt, &vn)?;

                        handle_face(v1, v2, v3, &mut obj, &mut unique_vertices, has_normal);
                    } else if line.len() == 5 {
                        let (v1, has_normal, _has_texture) = parse_face_el(line[1], &v, &vt, &vn)?;
                        let (v2, _, _) = parse_face_el(line[2], &v, &vt, &vn)?;
                        let (v3, _, _) = parse_face_el(line[3], &v, &vt, &vn)?;
                        let (v4, _, _) = parse_face_el(line[4], &v, &vt, &vn)?;

                        handle_face(v1, v2, v3, &mut obj, &mut unique_vertices, has_normal);
                        handle_face(v1, v3, v4, &mut obj, &mut unique_vertices, has_normal);
                    } else if line.len() > 5 {
                        println!(
                            "obj warning: line {}: faces with more than 5 vertices are not handled and are ignored",
                            line_number
                        );
                    } else {
                        return Err(format!(
                            "obj parsing error: line {line_number}: expected (a, b, c [, d]) format"
                        )
                        .into());
                    }
                }
                "#" | "o" | "s" | "mtllib" | "usemtl" | "g" => continue,
                _ => {
                    return Err(format!(
                        "obj parsing error: line {line_number}: invalid line start"
                    )
                    .into())
                }
            }
        }

        obj.set_obj_size_and_center();

        if vt.len() == 1 {
            obj.set_naive_texture_coordinate();
        }

        Ok(obj)
    }

    fn set_obj_size_and_center(&mut self) {
        let mut vmax = Vec3 {
            x: std::f32::MIN,
            y: std::f32::MIN,
            z: std::f32::MIN,
        };
        let mut vmin = Vec3 {
            x: std::f32::MAX,
            y: std::f32::MAX,
            z: std::f32::MAX,
        };
        for vertex in self.vertex.iter() {
            vmax = Vec3 {
                x: vmax.x.max(vertex.position[0]),
                y: vmax.y.max(vertex.position[1]),
                z: vmax.z.max(vertex.position[2]),
            };
            vmin = Vec3 {
                x: vmin.x.min(vertex.position[0]),
                y: vmin.y.min(vertex.position[1]),
                z: vmin.z.min(vertex.position[2]),
            };
        }
        self.size = Vec3 {
            x: vmax.x - vmin.x,
            y: vmax.y - vmin.y,
            z: vmax.z - vmin.z,
        };
        self.center = Vec3 {
            x: (vmin.x + vmax.x) / 2.0,
            y: (vmin.y + vmax.y) / 2.0,
            z: (vmin.z + vmax.z) / 2.0,
        };
    }

    fn set_naive_texture_coordinate(&mut self) {
        for vertex in self.vertex.iter_mut() {
            vertex.texture = [vertex.position[2] / self.size.z, -vertex.position[1] / self.size.y]
        }
    }
}

fn parse_face_el(
    face: &str,
    v: &[[[f32; 3]; 2]],
    vt: &[[f32; 2]],
    vn: &[[f32; 3]],
) -> Result<(Vertexxx, bool, bool), Box<dyn Error>> {
    let el: Vec<&str> = face.split('/').collect();

    match el.len() {
        // vertices only
        1 => {
            let vertex = Vertexxx {
                position: v[convert_index(el[0], v.len())?][0],
                color: v[convert_index(el[0], v.len())?][1],
                ..Default::default()
            };
            Ok((vertex, false, false))
        }
        // vertices and textures
        2 => {
            let vertex = Vertexxx {
                position: v[convert_index(el[0], v.len())?][0],
                color: v[convert_index(el[0], v.len())?][1],
                texture: vt[convert_index(el[1], vt.len())?],
                ..Default::default()
            };
            Ok((vertex, false, false))
        }
        3 => {
            // vertices, normals and textures
            if el[1] != "" {
                let vertex = Vertexxx {
                    position: v[convert_index(el[0], v.len())?][0],
                    color: v[convert_index(el[0], v.len())?][1],
                    texture: vt[convert_index(el[1], vt.len())?],
                    normal: vn[convert_index(el[2], vn.len())?],
                };
                Ok((vertex, true, true))
            // vertices and normals
            } else {
                let vertex = Vertexxx {
                    position: v[convert_index(el[0], v.len())?][0],
                    color: v[convert_index(el[0], v.len())?][1],
                    normal: vn[convert_index(el[2], vn.len())?],
                    ..Default::default()
                };
                Ok((vertex, true, false))
            }
        }
        _ => Err("Error parsing face".into()),
    }
}

fn convert_index(i: &str, size: usize) -> Result<usize, Box<dyn Error>> {
    let signed: i32 = i.parse()?;
    if signed < 0 {
        match size.checked_sub(signed.abs() as usize) {
            Some(unsigned) => Ok(unsigned),
            None => Err("obj parsing error: index error while parsing face".into()),
        }
    } else {
        Ok(signed as usize)
    }
}

fn handle_face(
    mut v1: Vertexxx,
    mut v2: Vertexxx,
    mut v3: Vertexxx,
    obj: &mut Object,
    unique_vertices: &mut HashMap<Vertexxx, u32>,
    has_normal: bool,
) {
    if !has_normal {
        let normal = calculate_normal(&v1, &v2, &v3);

        v1.normal = normal;
        v2.normal = normal;
        v3.normal = normal;
    }

    for v in [v1, v2, v3] {
        if unique_vertices.contains_key(&v) {
            obj.indice.push(unique_vertices[&v]);
        } else {
            unique_vertices.insert(v, obj.vertex.len() as u32);
            obj.indice.push(obj.vertex.len() as u32);
            obj.vertex.push(v);
        }
    }
}

fn calculate_normal(v1: &Vertexxx, v2: &Vertexxx, v3: &Vertexxx) -> [f32; 3] {
    let v1 = Vec3::from(&v1.position);
    let v2 = Vec3::from(&v2.position);
    let v3 = Vec3::from(&v3.position);

    Vec3::normalize(&Vec3::cross(&(v2 - v1), &(v3 - v2))).to_array()
}
