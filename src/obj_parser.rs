use core::fmt;

pub fn parse_obj(obj_data: String) -> Result<Vec<f32>, OBJParseError> {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut output: Vec<f32> = Vec::new();

    for line in obj_data.lines() {
        let mut words = line.split_whitespace();

        let Some(command_string) = words.next() else {
            continue;
        };

        let command = match Command::try_from(command_string) {
            Ok(command) => command,

            // Ignoring invalid commands
            Err(_) => {
                continue;
            }
        };

        match command {
            Command::Position => positions.push(parse_vector_3(words)?),
            Command::Normal => normals.push(parse_vector_3(words)?),
            Command::UV => uvs.push(parse_vector_2(words)?),

            Command::Face => {
                for face in words {
                    let face = parse_face(face)?;

                    // In .obj files, indices are 1-based instead of 0-based.
                    let position_index = face[0] as usize - 1;
                    let normal_index = face[2] as usize - 1;
                    let uv_index = face[1] as usize - 1;

                    let position = positions.get(position_index).ok_or(OBJParseError::InvalidFaceIndex)?;
                    let normal = normals.get(normal_index).ok_or(OBJParseError::InvalidFaceIndex)?;
                    let uv = uvs.get(uv_index).ok_or(OBJParseError::InvalidFaceIndex)?;

                    output.extend_from_slice(position);
                    output.extend_from_slice(normal);
                    output.extend_from_slice(uv);
                }
            }
        }
    }

    Ok(output)
}

fn parse_face(face: &str) -> Result<[u32; 3], OBJParseError> {
    let mut parts = face.split("/");

    let x = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;
    let y = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;
    let z = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;

    if parts.next().is_some() {
        return Err(OBJParseError::CommandHasTooManyParameters);
    }

    Ok([x, y, z])
}

fn parse_vector_3(mut parts: std::str::SplitWhitespace) -> Result<[f32; 3], OBJParseError> {
    let x: f32 = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;
    let y: f32 = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;
    let z: f32 = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;

    if parts.next().is_some() {
        return Err(OBJParseError::CommandHasTooManyParameters);
    }

    Ok([x, y, z])
}

fn parse_vector_2(mut parts: std::str::SplitWhitespace) -> Result<[f32; 2], OBJParseError> {
    let x: f32 = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;
    let y: f32 = parts.next().ok_or(OBJParseError::MissingData)?.parse()?;

    if parts.next().is_some() {
        return Err(OBJParseError::CommandHasTooManyParameters);
    }

    Ok([x, 1.0 - y])
}

enum Command {
    Position,
    Normal,
    UV,
    Face,
}

impl TryFrom<&str> for Command {
    type Error = OBJParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let command = match value {
            "v" => Command::Position,
            "vn" => Command::Normal,
            "vt" => Command::UV,
            "f" => Command::Face,
            _ => return Err(OBJParseError::UnsupportedCommand(String::from(value))),
        };

        Ok(command)
    }
}

#[derive(Debug)]
pub enum OBJParseError {
    MissingData,
    InvalidFaceIndex,
    UnsupportedCommand(String),
    CommandHasTooManyParameters,
    InvalidInt(std::num::ParseIntError),
    InvalidFloat(std::num::ParseFloatError),
}

impl fmt::Display for OBJParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OBJParseError::MissingData => write!(f, "Missing data for a command."),
            OBJParseError::InvalidFaceIndex => write!(f, "Failed to parse face index as a u32."),
            OBJParseError::UnsupportedCommand(cmd) => write!(f, "Unsupported command: {}", cmd),
            OBJParseError::CommandHasTooManyParameters => write!(f, "Command has too many parameters"),
            OBJParseError::InvalidInt(error) => write!(f, "Failed to parse a int value: {}", error),
            OBJParseError::InvalidFloat(error) => write!(f, "Failed to parse a float value: {}", error),
        }
    }
}

impl From<std::num::ParseFloatError> for OBJParseError {
    fn from(error: std::num::ParseFloatError) -> OBJParseError {
        OBJParseError::InvalidFloat(error)
    }
}

impl From<std::num::ParseIntError> for OBJParseError {
    fn from(error: std::num::ParseIntError) -> OBJParseError {
        OBJParseError::InvalidInt(error)
    }
}
