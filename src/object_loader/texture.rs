use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn parse_ppm(file: &str) -> Result<Self> {
        let lines: Vec<&str> = file
            .lines()
            .filter(|l| !l.starts_with("#") && !l.is_empty())
            .collect();

        if lines.len() < 3 || lines[0] != "P3" {
            return Err(anyhow!("Invalid ppm file"));
        }

        let size: Vec<&str> = lines[1].split(' ').collect();
        let mut texture = Texture {
            width: size[0].parse()?,
            height: size[1].parse()?,
            data: Vec::new(),
        };

        let max_value: u16 = lines[2].parse()?;

        for line in lines.iter().skip(3) {
            for value in line.split_ascii_whitespace() {
                let value = ((value.parse::<u16>()? / max_value) * 255) as u8;
                texture.data.push(value);
            }
        }

        if texture.data.len() as u32 != 3 * texture.height * texture.width {
            return Err(anyhow!(
                "The ppm file doesnt contain all the rgb values for its dimensions"
            ));
        }

        println!("{texture:?}");

        Ok(texture)
    }
}
