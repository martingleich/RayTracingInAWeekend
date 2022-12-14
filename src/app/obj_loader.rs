use std::str::Chars;

use ray_tracing_in_a_weekend::{
    triangle_geometry::TriangleGeometry, Dir3, Point3, Vec2, Vec2f, Vec3,
};

pub fn load_obj_mesh<R: std::io::BufRead>(reader: R) -> Result<Vec<TriangleGeometry>> {
    let mut loader = ObjLoader {
        positions: Vec::new(),
        normals: Vec::new(),
        texture_coords: Vec::new(),
        triangles: Vec::new(),
    };
    for maybe_line in reader.lines() {
        match maybe_line {
            Ok(line) => loader.parse_line(&line)?,
            Err(err) => return Err(ScannerError::IoError(err)),
        }
    }

    Ok(loader.triangles)
}

struct ObjLoader {
    positions: Vec<Point3>,
    normals: Vec<Dir3>,
    texture_coords: Vec<Vec2f>,
    triangles: Vec<TriangleGeometry>,
}

struct Scanner<'a> {
    chars: Chars<'a>,
    str: &'a str,
    peeked: Option<char>,
}

#[derive(Debug)]
pub enum ScannerError {
    IoError(std::io::Error),
    UnexpectedEndOfFile,
    ExpectedAtLeastOnWhitespace,
    ExpectedDigits,
    UsizeFormat(<usize as std::str::FromStr>::Err),
    F32Format(<f32 as std::str::FromStr>::Err),
    PosIdOutOfRange(usize),
    NorIdOutOfRange(usize),
    TexIdOutOfRange(usize),
}

type Vertex = (usize, Option<usize>, Option<usize>);
enum LineType {
    V(Vec3<f32>),
    Vn(Vec3<f32>),
    Vt(Vec2<f32>),
    F(Vec<Vertex>),
    Unknown,
    Empty,
    Comment,
    Object,
}

type Result<T> = std::result::Result<T, ScannerError>;

impl<'a> Scanner<'a> {
    pub fn new(mut chars: Chars<'a>) -> Scanner<'a> {
        let str = chars.as_str();
        let peeked = chars.next();
        Scanner { chars, str, peeked }
    }
    pub fn parse_line(chars: Chars<'a>) -> Result<LineType> {
        Self::new(chars).take_line_type()
    }

    pub fn try_take(&mut self) -> Option<char> {
        let result = self.peeked;
        self.str = self.chars.as_str();
        self.peeked = self.chars.next();
        result
    }
    pub fn try_take_char(&mut self, c: char) -> Option<char> {
        self.try_take_char_fn(|x| x == c)
    }
    pub fn try_take_char_fn<F: Fn(char) -> bool>(&mut self, c: F) -> Option<char> {
        if let Some(x) = self.peeked {
            if c(x) {
                self.take().unwrap();
                Some(x)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<char> {
        self.peeked
    }
    pub fn take(&mut self) -> Result<char> {
        if let Some(x) = self.try_take() {
            Ok(x)
        } else {
            Err(ScannerError::UnexpectedEndOfFile)
        }
    }
    pub fn take_at_least_one_whitespace(&mut self) -> Result<()> {
        if self.try_take_char(' ').is_none() {
            return Err(ScannerError::ExpectedAtLeastOnWhitespace);
        }
        self.take_any_whitespace()
    }
    pub fn take_any_whitespace(&mut self) -> Result<()> {
        while self.try_take_char(' ').is_some() {}
        Ok(())
    }
    pub fn take_str_digits(&mut self) -> Result<&str> {
        let start = self.str;
        if self.try_take_char_fn(char::is_numeric).is_none() {
            return Err(ScannerError::ExpectedDigits);
        }

        while self.try_take_char_fn(char::is_numeric).is_some() {}
        Ok(&start[..start.len() - self.str.len()])
    }
    pub fn take_usize(&mut self) -> Result<usize> {
        let digits = self.take_str_digits();
        digits.and_then(|str| str.parse::<usize>().map_err(ScannerError::UsizeFormat))
    }
    pub fn take_float_digits(&mut self) -> Result<&str> {
        let start = self.str;
        self.try_take_char('-');
        self.take_str_digits()?;
        if self.try_take_char('.').is_some() {
            self.take_str_digits()?;
        }
        Ok(&start[..start.len() - self.str.len()])
    }
    pub fn take_f32(&mut self) -> Result<f32> {
        let digits = self.take_float_digits();
        digits.and_then(|str| str.parse::<f32>().map_err(ScannerError::F32Format))
    }
    pub fn take_line_type(&mut self) -> Result<LineType> {
        match self.try_take_char_fn(|c| (c == 'v' || c == 'f' || c == '#' || c == 'o')) {
            Some('v') => match self.try_take_char_fn(|c| (c == 'n' || c == 't')) {
                Some('n') => {
                    let v = self.take_vec3_f32()?;
                    Ok(LineType::Vn(v))
                }
                Some('t') => {
                    let v = self.take_vec2_f32()?;
                    Ok(LineType::Vt(v))
                }
                _ => {
                    let v = self.take_vec3_f32()?;
                    Ok(LineType::V(v))
                }
            },
            Some('f') => {
                self.take_at_least_one_whitespace()?;
                let mut vertex_ids = Vec::new();
                while self.peek() != None {
                    let ids = self.take_vertex()?;
                    self.take_any_whitespace()?;
                    vertex_ids.push(ids);
                }
                Ok(LineType::F(vertex_ids))
            }
            Some('#') => Ok(LineType::Comment),
            Some('o') => Ok(LineType::Object),
            _ => Ok(LineType::Unknown),
        }
    }
    pub fn take_vec3_f32(&mut self) -> Result<Vec3<f32>> {
        self.take_at_least_one_whitespace()?;
        let x = self.take_f32()?;
        self.take_at_least_one_whitespace()?;
        let y = self.take_f32()?;
        self.take_at_least_one_whitespace()?;
        let z = self.take_f32()?;
        Ok(Vec3::new(x, y, z))
    }
    pub fn take_vec2_f32(&mut self) -> Result<Vec2f> {
        self.take_at_least_one_whitespace()?;
        let x = self.take_f32()?;
        self.take_at_least_one_whitespace()?;
        let y = self.take_f32()?;
        Ok(Vec2f::new(x, y))
    }
    pub fn take_vertex(&mut self) -> Result<(usize, Option<usize>, Option<usize>)> {
        let pos_id = self.take_usize()? - 1;
        self.take_any_whitespace()?;
        if self.try_take_char('/').is_some() {
            self.take_any_whitespace()?;
            if self.try_take_char('/').is_some() {
                self.take_any_whitespace()?;
                let nor_id = self.take_usize()? - 1;
                Ok((pos_id, None, Some(nor_id)))
            } else {
                let uv_id = self.take_usize()? - 1;
                self.take_any_whitespace()?;
                if self.try_take_char('/').is_some() {
                    self.take_any_whitespace()?;
                    let nor_id = self.take_usize()? - 1;
                    Ok((pos_id, Some(uv_id), Some(nor_id)))
                } else {
                    Ok((pos_id, Some(uv_id), None))
                }
            }
        } else {
            Ok((pos_id, None, None))
        }
    }
}

impl ObjLoader {
    fn parse_line(&mut self, str: &str) -> Result<()> {
        match Scanner::parse_line(str.chars())? {
            LineType::V(v) => {
                self.positions.push(Point3(v));
            }
            LineType::Vn(v) => {
                self.normals.push(Dir3(v));
            }
            LineType::Vt(v) => {
                self.texture_coords.push(v);
            }
            LineType::F(vertex_ids) => {
                let mut tri = TriangleGeometry {
                    positions: [Point3::ORIGIN, Point3::ORIGIN, Point3::ORIGIN],
                    normals: [Dir3::ZERO, Dir3::ZERO, Dir3::ZERO],
                    texture_coords: [Vec2f::ZERO, Vec2f::ZERO, Vec2f::ZERO],
                };
                for (i, ids) in vertex_ids.iter().enumerate() {
                    let position = *self
                        .positions
                        .get(ids.0)
                        .ok_or(ScannerError::PosIdOutOfRange(ids.0))?;
                    let uv = match ids.1 {
                        Some(i) => *self
                            .texture_coords
                            .get(i)
                            .ok_or(ScannerError::TexIdOutOfRange(i))?,
                        None => Vec2f::ZERO,
                    };
                    let normal = match ids.2 {
                        Some(i) => *self
                            .normals
                            .get(i)
                            .ok_or(ScannerError::NorIdOutOfRange(i))?,
                        None => todo!(),
                    };
                    if i < 2 {
                        tri.positions[i] = position;
                        tri.normals[i] = normal;
                        tri.texture_coords[i] = uv;
                    } else {
                        tri.positions[1] = tri.positions[2];
                        tri.normals[1] = tri.normals[2];
                        tri.texture_coords[1] = tri.texture_coords[2];

                        tri.positions[2] = position;
                        tri.normals[2] = normal;
                        tri.texture_coords[2] = uv;
                        self.triangles.push(tri);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
