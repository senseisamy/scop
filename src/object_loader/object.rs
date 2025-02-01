use super::{Object, Vertexxx};
use crate::math::Vec3;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct ParseObjectError;

impl std::str::FromStr for Object {
    type Err = ParseObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<[f32; 3]> = Vec::from([[0.0, 0.0, 0.0]]);
        let mut vt: Vec<[f32; 3]> = Vec::from([[0.0, 0.0, 0.0]]);
        let mut vn: Vec<[f32; 3]> = Vec::from([[0.0, 0.0, 0.0]]);

        let mut unique_vertices: HashMap<Vertexxx, u16> = HashMap::new();
        let mut obj = Object {
            vertex: Vec::from([Vertexxx::default()]),
            indice: Vec::new(),
        };

        for line in s.lines() {
            let line: Vec<&str> = line
                .split_ascii_whitespace()
                .take_while(|x| !x.contains("#"))
                .collect();
            if line.len() < 2 {
                continue;
            }
            match line[0] {
                "v" => {
                    if line.len() != 4 {
                        return Err(ParseObjectError);
                    }
                    v.push([
                        line[1].parse().map_err(|_| ParseObjectError)?,
                        line[2].parse().map_err(|_| ParseObjectError)?,
                        line[3].parse().map_err(|_| ParseObjectError)?,
                    ]);
                }
                "vt" => {
                    if line.len() != 3 {
                        return Err(ParseObjectError);
                    }
                    vt.push([
                        line[1].parse().map_err(|_| ParseObjectError)?,
                        line[2].parse().map_err(|_| ParseObjectError)?,
                        0.0,
                    ]);
                }
                "vn" => {
                    if line.len() != 4 {
                        return Err(ParseObjectError);
                    }
                    vn.push([
                        line[1].parse().map_err(|_| ParseObjectError)?,
                        line[2].parse().map_err(|_| ParseObjectError)?,
                        line[3].parse().map_err(|_| ParseObjectError)?,
                    ]);
                }
                "f" => {
                    if line.len() == 4 {
                        let (v1, has_normal, _has_texture) =
                            parse_face_el(line[1], &v, &vt, &vn).map_err(|_| ParseObjectError)?;
                        let (v2, _, _) =
                            parse_face_el(line[2], &v, &vt, &vn).map_err(|_| ParseObjectError)?;
                        let (v3, _, _) =
                            parse_face_el(line[3], &v, &vt, &vn).map_err(|_| ParseObjectError)?;

                        handle_face(v1, v2, v3, &mut obj, &mut unique_vertices, has_normal);
                    } else if line.len() == 5 {
                        let (v1, has_normal, _has_texture) =
                            parse_face_el(line[1], &v, &vt, &vn).map_err(|_| ParseObjectError)?;
                        let (v2, _, _) =
                            parse_face_el(line[2], &v, &vt, &vn).map_err(|_| ParseObjectError)?;
                        let (v3, _, _) =
                            parse_face_el(line[3], &v, &vt, &vn).map_err(|_| ParseObjectError)?;
                        let (v4, _, _) =
                            parse_face_el(line[4], &v, &vt, &vn).map_err(|_| ParseObjectError)?;

                        handle_face(v1, v2, v3, &mut obj, &mut unique_vertices, has_normal);
                        handle_face(v2, v3, v4, &mut obj, &mut unique_vertices, has_normal);
                    } else {
                        return Err(ParseObjectError);
                    }
                }
                "#" | "o" | "s" | "mtllib" | "usemtl" | "g" => continue,
                _ => return Err(ParseObjectError),
            }
        }

        Ok(obj)
    }
}

fn parse_face_el(
    face: &str,
    v: &[[f32; 3]],
    vt: &[[f32; 3]],
    vn: &[[f32; 3]],
) -> Result<(Vertexxx, bool, bool)> {
    let el: Vec<&str> = face.split('/').collect();

    match el.len() {
        1 => {
            let vertex = Vertexxx {
                position: v[el[0].parse::<usize>()?],
                ..Default::default()
            };
            Ok((vertex, false, false))
        }
        2 => {
            let vertex = Vertexxx {
                position: v[el[0].parse::<usize>()?],
                texture: vt[el[1].parse::<usize>()?],
                ..Default::default()
            };
            Ok((vertex, false, false))
        }
        3 => {
            if el[1] != "" {
                let vertex = Vertexxx {
                    position: v[el[0].parse::<usize>()?],
                    texture: vt[el[1].parse::<usize>()?],
                    normal: vn[el[2].parse::<usize>()?],
                };
                Ok((vertex, true, true))
            } else {
                let vertex = Vertexxx {
                    position: v[el[0].parse::<usize>()?],
                    normal: vn[el[2].parse::<usize>()?],
                    ..Default::default()
                };
                Ok((vertex, true, false))
            }
        }
        _ => Err(anyhow!("Parsing error")),
    }
}

fn handle_face(
    mut v1: Vertexxx,
    mut v2: Vertexxx,
    mut v3: Vertexxx,
    obj: &mut Object,
    unique_vertices: &mut HashMap<Vertexxx, u16>,
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
            unique_vertices.insert(v, obj.vertex.len() as u16);
            obj.indice.push(obj.vertex.len() as u16);
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
