use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex, Debug, Clone)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Object {
    pub vertex: Vec<Position>,
    pub indice: Vec<u16>,
}

pub enum Face {
    Point(u16),
    Line(u16, u16),
    Triangle(u16, u16, u16),
    Quad(u16, u16, u16, u16),
    Polygon(Vec<u16>),
}

pub struct ParseObjectError;

impl std::str::FromStr for Object {
    type Err = ParseObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obj = Self {
            vertex: vec![Position {
                position: [0.0, 0.0, 0.0],
            }],
            indice: Vec::new(),
        };
        let mut faces: Vec<Face> = Vec::new();

        for line in s.lines() {
            let line: Vec<&str> = line.split_ascii_whitespace().collect();
            if line.len() < 2 {
                continue;
            }
            match line[0] {
                "v" => {
                    if line.len() != 4 {
                        return Err(ParseObjectError);
                    }
                    let vertex = Position {
                        position: [
                            line[1].parse().map_err(|_| ParseObjectError)?,
                            line[2].parse().map_err(|_| ParseObjectError)?,
                            line[3].parse().map_err(|_| ParseObjectError)?,
                        ],
                    };
                    obj.vertex.push(vertex);
                }
                "f" => {
                    let mut l: Vec<u16> = Vec::new();
                    for s in line.into_iter().skip(1) {
                        match s.parse::<u16>() {
                            Ok(n) => l.push(n),
                            Err(_) => return Err(ParseObjectError),
                        }
                    }
                    let face = match l.len() {
                        1 => Face::Point(l[0]),
                        2 => Face::Line(l[0], l[1]),
                        3 => Face::Triangle(l[0], l[1], l[2]),
                        4 => Face::Quad(l[0], l[1], l[2], l[3]),
                        _ => Face::Polygon(l),
                    };
                    faces.push(face);
                }
                "#" | "o" | "s" | "mtllib" | "usemtl" => continue,
                _ => return Err(ParseObjectError),
            }
        }

        obj.indice = convert_faces_to_triangles(&faces);

        Ok(obj)
    }
}

fn convert_faces_to_triangles(faces: &[Face]) -> Vec<u16> {
    let mut indices: Vec<u16> = Vec::new();
    let mut warn_unsupported = false;

    for face in faces {
        match face {
            Face::Point(_) => warn_unsupported = true,
            Face::Line(_, _) => warn_unsupported = true,
            Face::Triangle(a, b, c) => {
                indices.push(*a);
                indices.push(*b);
                indices.push(*c);
            }
            Face::Quad(a, b, c, d) => {
                indices.push(*a);
                indices.push(*b);
                indices.push(*c);

                indices.push(*b);
                indices.push(*c);
                indices.push(*d);
            }
            Face::Polygon(_) => warn_unsupported = true,
        }
    }

    if warn_unsupported {
        println!("Warning: points, lines and polygons in object file are not supported")
    }

    indices
}
