use super::{Terrain, Tile, TileType};
use nalgebra::Vector2;
#[derive(PartialEq, Debug)]
pub enum Context {
    XDimension,
    YDimension,
    MaxHeight,
    Height,
}
#[derive(PartialEq, Debug)]
pub enum ParseError {
    InvalidMagicNumber(String),
    EmptyFile,
    InvalidNumber { context: Context, error: String },
    MissingXDimension,
    MissingYDimension,
    MissingMaxHeight,
    MissingDatapoint,
}
pub fn terrain_from_pgm(data: String, default_tile_type: TileType) -> Result<Terrain, ParseError> {
    let mut iter = SkipWhitespace::new(data.as_str());
    if let Some(magic_number) = iter.next() {
        if magic_number != "P2" {
            return Err(ParseError::InvalidMagicNumber(magic_number));
        }
    } else {
        return Err(ParseError::EmptyFile);
    }
    let x_dimension_string = if let Some(s) = iter.next() {
        s
    } else {
        return Err(ParseError::MissingXDimension);
    };
    let y_dimension_string = if let Some(s) = iter.next() {
        s
    } else {
        return Err(ParseError::MissingYDimension);
    };
    let x_dimensions: usize = if let Some(x) = x_dimension_string.parse().ok() {
        x
    } else {
        return Err(ParseError::InvalidNumber {
            context: Context::XDimension,
            error: x_dimension_string,
        });
    };
    let y_dimensions: usize = if let Some(y) = y_dimension_string.parse().ok() {
        y
    } else {
        return Err(ParseError::InvalidNumber {
            context: Context::YDimension,
            error: y_dimension_string,
        });
    };
    let max_height_string = if let Some(s) = iter.next() {
        s
    } else {
        return Err(ParseError::MissingMaxHeight);
    };
    let max_height: usize = if let Some(h) = max_height_string.parse().ok() {
        h
    } else {
        return Err(ParseError::InvalidNumber {
            context: Context::MaxHeight,
            error: max_height_string,
        });
    };
    let mut tiles = vec![];
    tiles.reserve(x_dimensions * y_dimensions);
    for _x in 0..x_dimensions {
        for _y in 0..y_dimensions {
            let height_string = if let Some(s) = iter.next() {
                s
            } else {
                return Err(ParseError::MissingDatapoint);
            };
            let height: usize = if let Some(i) = height_string.parse().ok() {
                i
            } else {
                return Err(ParseError::InvalidNumber {
                    context: Context::Height,
                    error: height_string,
                });
            };
            tiles.push(Tile {
                height: height as f32,
                tile_type: default_tile_type.clone(),
            });
        }
    }
    Ok(Terrain::from_tiles(
        tiles,
        Vector2::new(x_dimensions, y_dimensions),
    ))
}
///Iterator over whitespace skips comments and whitespace characters
struct SkipWhitespace<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
}
impl<'a> SkipWhitespace<'a> {
    pub fn new(data: &'a str) -> Self {
        SkipWhitespace {
            iter: data.chars().peekable(),
        }
    }
}
fn is_white_space(c: char) -> bool {
    c == '\n' || c == ' ' || c == '\t'
}
impl<'a> SkipWhitespace<'a> {
    fn is_empty(s: String) -> Option<String> {
        if s == "" {
            Some(s)
        } else {
            None
        }
    }
    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.iter.peek() {
                if is_white_space(c.clone()) {
                    self.iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
impl<'a> Iterator for SkipWhitespace<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut string_out = String::new();
        loop {
            if let Some(c) = self.iter.next() {
                if is_white_space(c) {
                    loop {
                        if let Some(c) = self.iter.peek() {
                            if is_white_space(c.clone()) {
                                self.iter.next();
                            } else if c == &'#' {
                                loop {
                                    if let Some(c) = self.iter.peek() {
                                        if c == &'\n' {
                                            self.iter.next();
                                            break;
                                        } else {
                                            self.iter.next();
                                        }
                                    } else if string_out == "" {
                                        return None;
                                    } else {
                                        return Some(string_out);
                                    }
                                }
                                break;
                            } else {
                                break;
                            }
                        } else {
                            if string_out == "" {
                                return None;
                            } else {
                                return Some(string_out);
                            }
                        }
                    }
                    return Some(string_out);
                } else if c == '#' {
                    loop {
                        if let Some(c) = self.iter.next() {
                            if c == '\n' {
                                self.skip_whitespace();
                                return Self::is_empty(string_out);
                            }
                        } else {
                            if string_out == "" {
                                return None;
                            } else {
                                return Some(string_out);
                            }
                        }
                    }
                } else {
                    string_out.push(c);
                }
            } else {
                if string_out == "" {
                    return None;
                } else {
                    return Some(string_out);
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::super::{Terrain, Tile, TileType};
    use super::*;
    use nalgebra::Vector2;
    #[test]
    fn test_iterator() {
        let s = "s\ns2\n s3\n#do not read\ns4";
        let s_v: Vec<String> = SkipWhitespace::new(s).collect();
        assert_eq!(s_v, vec!["s", "s2", "s3", "s4"]);
    }
    #[test]
    fn test_iterator_spaces() {
        let s = "s1\n    s2";
        let s_v: Vec<String> = SkipWhitespace::new(s).collect();
        assert_eq!(s_v, vec!["s1", "s2"]);
    }
    #[test]
    fn test_iterator_pgm() {
        let s = "P2
    1 1
    10000
    10
            ";
        let s_v: Vec<String> = SkipWhitespace::new(s).collect();
        assert_eq!(s_v, vec!["P2", "1", "1", "10000", "10"]);
    }
    #[test]
    fn test_pgm_comment() {
        let s = "P2
    1 1
    #hello!
    10000
    10000
            ";
        let s_v: Vec<String> = SkipWhitespace::new(s).collect();
        assert_eq!(s_v, vec!["P2", "1", "1", "10000", "10"]);
    }

    #[test]
    fn basic_terrain() {
        let terrain = terrain_from_pgm(
            "P2
    1 1
    10000
    10000
            "
            .to_string(),
            TileType::Snow,
        );
        assert_eq!(
            terrain,
            Ok(Terrain::from_tiles(
                vec![Tile {
                    height: 10_000.0,
                    tile_type: TileType::Snow
                }],
                Vector2::new(1, 1)
            ))
        );
    }
    #[test]
    fn comment() {
        let terrain = terrain_from_pgm(
            "P2
    1 1
    #hello!
    10000
    10000
            "
            .to_string(),
            TileType::Snow,
        );
        assert_eq!(
            terrain,
            Ok(Terrain::from_tiles(
                vec![Tile {
                    height: 10_000.0,
                    tile_type: TileType::Snow
                }],
                Vector2::new(1, 1)
            ))
        );
    }
}
