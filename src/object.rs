#[derive(Default, Debug)]
pub struct Vertex(f64, f64, f64);

#[derive(Debug)]
pub struct Object {
    pub vertexes: Vec<Vertex>,
    pub vertexes_indices: Vec<Vec<usize>>,
}

pub struct ParseObjectError;

impl std::str::FromStr for Object {
    type Err = ParseObjectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vertexes: Vec<Vertex> = Vec::new();
        let mut vertexes_indices: Vec<Vec<usize>> = Vec::new();

        for line in s.lines() {
            let line: Vec<&str> = line.split_ascii_whitespace().collect();
            match line[0] {
                "v" => {
                    if line.len() != 4 {
                        return Err(ParseObjectError);
                    }
                    let mut vertex = Vertex::default();
                    vertex.0 = line[1].parse().map_err(|_| ParseObjectError)?;
                    vertex.1 = line[2].parse().map_err(|_| ParseObjectError)?;
                    vertex.2 = line[3].parse().map_err(|_| ParseObjectError)?;
                    vertexes.push(vertex);
                }
                "f" => {
                    if line.len() < 4 {
                        return Err(ParseObjectError);
                    }
                    let mut vertex_indices = Vec::new();
                    for indice in line.into_iter().skip(1) {
                        let indice = indice.parse::<usize>().map_err(|_| ParseObjectError)? - 1;
                        vertex_indices.push(indice);
                    }
                    vertexes_indices.push(vertex_indices);
                }
                "#" | "o" | "s" | "mtllib" | "usemtl" => continue,
                _ => return Err(ParseObjectError),
            }
        }

        Ok(Object {
            vertexes,
            vertexes_indices,
        })
    }
}
