
macro_rules! chunk
{
    // chunk with children
    (
        $( $name:ident [ $num:expr ] -> [ $($dep:ident),*] ),*
    ) => {

        #[repr(u16)]
        #[derive(Copy,Clone,Debug,PartialEq,Eq)]
        pub enum ChunkKind
        {
            $( $name = $num ),*
        }

        impl ChunkKind
        {
            pub fn from_u16(val: u16) -> Option<Self> {
                match val {
                    $( $num => Some(ChunkKind::$name) ),*
                    ,_ => None,
                }
            }

            /// Gets the possible child chunks that the chunk may have.
            pub fn possible_children(self) -> ::std::vec::IntoIter<ChunkKind> {
                match self {
                    $( ChunkKind::$name => vec![ $( ChunkKind::$dep),* ]),*
                }.into_iter()
            }
        }
    }
}

chunk!(
    Main[0x4D4D] -> [Version, Editor, Keyframer],
    Version[0x0002] -> [],
    Editor[0x3D3D] -> [ObjectBlock, MaterialBlock],
    ObjectBlock[0x4000] -> [TriangularMesh, Light, Camera],
    TriangularMesh[0x4100] -> [
        VerticesList, FacesDescription,
        MappingCoordinatesList, LocalCoordinatesSystem
    ],
    VerticesList[0x4110] -> [],
    FacesDescription[0x4120] -> [FacesMaterial, SmoothingGroupList],
    FacesMaterial[0x4130] -> [],
    SmoothingGroupList[0x4150] -> [],
    MappingCoordinatesList[0x4140] -> [],
    LocalCoordinatesSystem[0x4160] -> [],
    Light[0x4600] -> [Spotlight],
    Spotlight[0x4610] -> [],
    Camera[0x4700] -> [],
    MaterialBlock[0xAFFF] -> [
        MaterialName, AmbientColor, DiffuseColor,
        SpecularColor, TextureMap1, BumpMap,
        ReflectionMap
    ],
    MaterialName[0xA000] -> [],
    AmbientColor[0xA010] -> [],
    DiffuseColor[0xA020] -> [],
    SpecularColor[0xA030] -> [],
    TextureMap1[0xA200] -> [],
    BumpMap[0xA230] -> [],
    ReflectionMap[0xA220] -> [MappingFilename, MappingParameters],
    MappingFilename[0xA300] -> [],
    MappingParameters[0xA351] -> [],
    Keyframer[0xB000] -> [
        MeshInformationBlock, SpotlightInformationBlock,
        Frames
    ],
    MeshInformationBlock[0xB002] -> [],
    SpotlightInformationBlock[0xB007] -> [],
    Frames[0xB008] -> [
        ObjectName, ObjectPivotPoint, PositionTrack,
        RotationTrack, ScaleTrack, HierarchyPosition
    ],
    ObjectName[0xB010] -> [],
    ObjectPivotPoint[0xB013] -> [],
    PositionTrack[0xB020] -> [],
    RotationTrack[0xB021] -> [],
    ScaleTrack[0xB022] -> [],
    HierarchyPosition[0xB030] -> []
);

/// Mid-level view of a 3DS file.
pub mod mid
{
    use super::ChunkKind;
    use super::low;
    use std::io;

    pub struct Chunk
    {
        kind: ChunkKind,
        children: Vec<Chunk>,
    }

    impl Chunk
    {
        fn from_low(chunk: low::Chunk) -> Self {
            Chunk {
                kind: ChunkKind::from_u16(chunk.id).unwrap(),
                children: Vec::new(),
            }
        }
    }
}

/// Low-level view of a 3DS file.
pub mod low
{
    use std::io;
    use byteorder::{LittleEndian,ReadBytesExt};

    pub fn read(read: &mut io::Read) -> Result<Vec<Chunk>,io::Error> {
        unimplemented!();
    }

    /// A 3Ds chunk.
    pub struct Chunk
    {
        pub id: u16,
        pub next_ptr: u32,
    }

    impl Chunk
    {
        pub fn read(read: &mut io::Read) -> Result<Self,io::Error> {

            let id = try!(read.read_u16::<LittleEndian>());
            let next_ptr = try!(read.read_u32::<LittleEndian>());

            Ok(Chunk {
                id: id,
                next_ptr: next_ptr,
            })
        }
    }
}
