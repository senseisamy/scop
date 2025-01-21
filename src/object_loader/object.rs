use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex, Debug)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3]
}

#[derive(Debug)]
pub struct Object {
    pub vertex: Vec<Position>,
    pub indice: Vec<u16>
}

pub struct ParseObjectError;

impl std::str::FromStr for Object {
    type Err = ParseObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obj = Self {
            vertex: vec![Position {
                position: [0.0, 0.0, 0.0]
            }],
            indice: Vec::new()
        };

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
                            line[3].parse().map_err(|_| ParseObjectError)?
                        ]
                    };
                    obj.vertex.push(vertex);
                }
                "f" => {
                    if line.len() < 4 {
                        return Err(ParseObjectError);
                    }
                    for indice in line.into_iter().skip(1) {
                        let indice = indice.parse::<u16>().map_err(|_| ParseObjectError)?;
                        obj.indice.push(indice);
                    }
                }
                "#" | "o" | "s" | "mtllib" | "usemtl" => continue,
                _ => return Err(ParseObjectError),
            }
        }

        Ok(obj)
    }
}
