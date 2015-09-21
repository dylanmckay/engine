
use math::{Scalar,Vector3};
use geom::{self,mesh,util,Format};
use num;

use std::io;

// TODO: parse VertexTextureCoords

#[derive(Copy,Clone)]
pub struct Vertex
{
    pub position: Vector3,
    pub normal: Option<Vector3>,
    pub uv: Option<(Scalar,Scalar)>,
}

pub type Face = Vec<(i32,i32,i32)>;

impl geom::Vertex for Vertex
{
    type T = Scalar;

    fn coords(self) -> Vector3<Scalar> {
        self.position
    }
}

impl Vertex
{
    pub fn new(position: Vector3,
               normal: Option<Vector3>,
               uv: Option<(Scalar,Scalar)>) -> Self {
        Vertex {
            position: position,
            normal: normal,
            uv: uv,
        }
    }
}

// Gets the position of a vertex
impl<T: num::Num> From<Vertex> for Vector3<T> {
    fn from(vert: Vertex) -> Vector3<T> {
        vert.position.cast()
    }
}

pub struct Wavefront;

impl<I,V> Format<I,V> for Wavefront
    where I: num::Integer,
          V: From<Vertex>
{
    fn load_with_builder<R>(read: R, builder: &mut mesh::Builder<I,V>)
        where R: io::Read {
        use std::io::BufRead;

        let reader = io::BufReader::new(read);

        let mut points = Vec::new();
        let mut normals =Vec::new();
        let mut uvs = Vec::new();
        let mut face_indices: Vec<Face> = Vec::new();

        // We find the pairs of position/normal/uv vertices and group
        // them into here. Note that `-1` is considered an unspecified value.
        // We later sort and deduplicate.
        let mut distinct_vertices: Vec<(i32,i32,i32)> = Vec::new();

        let statements = reader.lines()
                               .map(|a| self::load::parse_line(&a.unwrap()));

        // store relevant mesh data into their respective arrays
        for stmt in statements {
            match stmt {
                Statement::Vertex(x,y,z,w) => {
                    points.push((x,y,z,w))
                },
                Statement::VertexNormal(x,y,z) => {
                    normals.push((x,y,z))
                },
                Statement::VertexTextureCoords(u,v) => {
                    uvs.push((u,v))
                },
                Statement::Face(f) => {
                    for &vertex in f.iter() {
                        distinct_vertices.push(vertex);
                    }
                    face_indices.push(f);
                },
                _ => (),
            }
        }

        // Remove all duplicate vertices, so we are only storing
        // unique values.
        distinct_vertices.sort();
        distinct_vertices.dedup();

        // create the final vertex array
        let vertex_index_map: Vec<((i32,i32,i32),Vertex)> = distinct_vertices.iter()
                                                     .map(|&(v,vn,vt)| {
                                                         let (px,py,pz,_) = points[v as usize];
                                                         let point = Vector3(px,py,pz);

                                                         let normal = match vn {
                                                             -1 => None,
                                                             _  => Some(normals[vn as usize].into()),
                                                         };
             
                                                         let uv = match vt {
                                                             -1 => None,
                                                             _  => Some(uvs[vt as usize]),
                                                         };

                                                         ((v,vn,vt), Vertex::new(point, normal, uv))
                                                     })
                                                     .collect();

        drop(points);
        drop(normals);
        drop(uvs);

        let faces: Vec<Vec<I>> = face_indices.into_iter()
                                                .map(|vec| {
            vec.into_iter().map(|(v,vn,vt)| {
                    num::cast(vertex_index_map.iter().position(|&((fv,fvn,fvt),_)| {
                        (v == fv) && (vn == fvn) && (vt == fvt)
                    }).unwrap())
            }).collect()

        }).collect();

        let vertices = vertex_index_map.into_iter().map(|((_,_,_),v)| v.into());
        builder.feed_vertices(vertices);

        for face in faces {
            for triangulated_face in util::triangulate(face) {
                use geom::Face;

                // TODO: triangulate the face
                // currently we only accept triangular faces
                assert!(triangulated_face.is_triangular());

                builder.feed_indices(triangulated_face.into_iter());

            }
        }

    }
}

/// A statement in a Wavefront file
pub enum Statement {
    Vertex(f32, f32, f32, Option<f32>),
    VertexNormal(f32, f32, f32),
    VertexTextureCoords(f32, f32),
    /// A face.
    /// Note that indices are zero-based.
    Face(Vec<(i32,i32,i32)>),
    Object(Option<String>),
    SmoothShading(bool),
    /// An empty line.
    Empty,
}

pub mod load
{
    use std::str::FromStr;
    use geom::formats::wavefront::Statement;


    pub fn parse_line(line: &str) -> Statement {
        let mut words = line.split_whitespace();

        let kind = match words.next() {
            Some(k) => k,
            None => { return Statement::Empty; },
        };

        match kind {
            "#" =>  Statement::Empty,
            "v" =>  self::parse_vertex(words),
            "vn" => self::parse_vertex_normal(words),
            "f" =>  self::parse_face(words),
            "o" =>  self::parse_object(words),
            "s" =>  self::parse_smooth_shading(words),
            _ => panic!(format!("unimplemented wavefront statement kind: '{}'", kind)),
        }
    }

    pub fn parse_vertex<'a,I>(mut words: I) -> Statement
        where I: Iterator<Item=&'a str> {
        let x = FromStr::from_str(words.next().unwrap()).unwrap();
        let y = FromStr::from_str(words.next().unwrap()).unwrap();
        let z = FromStr::from_str(words.next().unwrap()).unwrap();
        let w = words.next().map(|a| FromStr::from_str(a).unwrap());

        Statement::Vertex(x,y,z,w)
    }

    pub fn parse_vertex_normal<'a,I>(words: I) -> Statement
        where I: Iterator<Item=&'a str> {
        let coords: Vec<f32> = words.take(3).map(|a| FromStr::from_str(a).unwrap()).collect();

        Statement::VertexNormal(coords[0], coords[1], coords[2])
    }


    pub fn parse_face<'a,I>(words: I) -> Statement
        where I: Iterator<Item=&'a str> {

        let indices = words.map(|word| parse_face_indices(word)).collect();

        Statement::Face(indices)
        
    }

    pub fn parse_face_indices(s: &str) -> (i32,i32,i32) {
        let indices: Vec<i32> = s.split('/')
                                 .map(|s| match s.trim().len() {
                                     // if it doesn't exist, set it to -1
                                     0 => -1,
                                     // parse the int
                                     _ => FromStr::from_str(s).unwrap(),
                                 }).collect();

        assert!(indices.len() <= 3);

        // get the indices. make sure they are zero based.
        let vi = indices.get(0).map(|&a|a-1).unwrap();
        let ni = indices.get(2).map(|&a| if a >= 0 { a-1 } else { a }).unwrap();
        let ti = indices.get(1).map(|&a| if a >= 0 { a-1 } else { a }).unwrap();

        (vi,ni,ti)
    }

    pub fn parse_object<'a,I>(mut words: I) -> Statement
        where I: Iterator<Item=&'a str> {
        let name: Option<String> = words.next().map(|a| a.into());

        if words.next().is_some() {
            panic!("expected end of line");
        }

        Statement::Object(name)
    }
    
    pub fn parse_smooth_shading<'a,I>(mut words: I) -> Statement
        where I: Iterator<Item=&'a str> {

        let enabled = match words.next().unwrap() {
            "0" | "off" => false,
            "1" | "on"  => true,
            _ => panic!("invalid boolean"),
        };

        Statement::SmoothShading(enabled)
    }
}
