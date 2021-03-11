// Do not edit: This code was generated by flatdata's generator.
#[allow(missing_docs)]
pub mod mesh {
#[repr(transparent)]
#[derive(Clone)]
pub struct Vector3 {
    data: [u8; 12],
}

impl Vector3 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 12]}
    }
}

impl flatdata::Struct for Vector3 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 12]}
    }

    const SIZE_IN_BYTES: usize = 12;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Vector3 {
    pub fn new( ) -> Self {
        Self{data : [0; 12]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 12]) -> &Self {
        // Safety: This is safe since Vector3 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 12]) -> &mut Self {
        // Safety: This is safe since Vector3 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 12 {
            assert_eq!(data.len(), 12);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 12];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 12 {
            assert_eq!(data.len(), 12);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 12];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.data
    }
}

impl Default for Vector3 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for Vector3 {}

impl Vector3 {
    #[inline]
    pub fn x(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn y(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 32, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn z(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 64, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}

impl std::cmp::PartialEq for Vector3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() &&        self.y() == other.y() &&        self.z() == other.z()     }
}

impl Vector3 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_x(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 32)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn set_y(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 32, 32)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn set_z(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 64, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Vector3) {
        self.set_x(other.x());
        self.set_y(other.y());
        self.set_z(other.z());
    }
}



#[derive(Clone)]
pub struct Mesh {
    _storage: flatdata::StorageHandle,
    triangles : &'static [super::mesh::Vector3],
}

impl Mesh {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    // Each triangle consists of 4 Vector3
    #[inline]
    pub fn triangles(&self) -> &[super::mesh::Vector3] {
        self.triangles
    }

}

impl ::std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Mesh")
            .field("triangles", &self.triangles())
            .finish()
    }
}

impl Mesh {
    pub fn open(storage: flatdata::StorageHandle)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use flatdata::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], flatdata::ResourceStorageError>| -> Result<&'static [u8], flatdata::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name("Mesh"), schema::mesh::MESH)?;

        let resource = extend(storage.read("triangles", schema::mesh::resources::TRIANGLES));
        let triangles = resource.map(|x| <&[super::mesh::Vector3]>::from_bytes(x))??;

        Ok(Self {
            _storage: storage,
            triangles,
        })
    }
}

/// Builder for creating [`Mesh`] archives.
///
///[`Mesh`]: struct.Mesh.html
#[derive(Clone, Debug)]
pub struct MeshBuilder {
    storage: flatdata::StorageHandle
}

impl MeshBuilder {
    #[inline]
    /// Stores [`triangles`] in the archive.
    ///
    /// [`triangles`]: struct.Mesh.html#method.triangles
    pub fn set_triangles(&self, vector: &[super::mesh::Vector3]) -> ::std::io::Result<()> {
        use flatdata::SliceExt;
        self.storage.write("triangles", schema::mesh::resources::TRIANGLES, vector.as_bytes())
    }

    /// Opens [`triangles`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`triangles`]: struct.Mesh.html#method.triangles
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_triangles(&self) -> ::std::io::Result<flatdata::ExternalVector<super::mesh::Vector3>> {
        flatdata::create_external_vector(&*self.storage, "triangles", schema::mesh::resources::TRIANGLES)
    }

}

impl MeshBuilder {
    pub fn new(
        storage: flatdata::StorageHandle,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive("Mesh", schema::mesh::MESH, &storage)?;
        Ok(Self { storage })
    }
}


#[doc(hidden)]
pub mod schema {
pub mod mesh {

pub const MESH: &str = r#"namespace mesh {
struct Vector3
{
    x : u32 : 32;
    y : u32 : 32;
    z : u32 : 32;
}
}

namespace mesh {
archive Mesh
{
    triangles : vector< .mesh.Vector3 >;
}
}

"#;

pub mod resources {
pub const TRIANGLES: &str = r#"namespace mesh {
struct Vector3
{
    x : u32 : 32;
    y : u32 : 32;
    z : u32 : 32;
}
}

namespace mesh {
archive Mesh
{
    triangles : vector< .mesh.Vector3 >;
}
}

"#;
}
}
}
}
