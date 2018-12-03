use ffi;
use std::mem;

pub fn generate_vertex_remap<T>(vertices: &[T], indices: Option<&[u32]>) -> (usize, Vec<u32>) {
    let remap: Vec<u32> = vec![0; vertices.len()];
    let vertex_count = unsafe {
        match indices {
            Some(indices) => ffi::meshopt_generateVertexRemap(
                remap.as_ptr() as *mut ::std::os::raw::c_uint,
                indices.as_ptr() as *const ::std::os::raw::c_uint,
                indices.len(),
                vertices.as_ptr() as *const ::std::os::raw::c_void,
                vertices.len(),
                mem::size_of::<T>(),
            ),
            None => ffi::meshopt_generateVertexRemap(
                remap.as_ptr() as *mut ::std::os::raw::c_uint,
                ::std::ptr::null(),
                vertices.len(),
                vertices.as_ptr() as *const ::std::os::raw::c_void,
                vertices.len(),
                mem::size_of::<T>(),
            ),
        }
    };
    (vertex_count, remap)
}

/// Generate index buffer from the source index buffer and remap table generated by generate_vertex_remap
///
/// indices can be `None` if the input is unindexed
pub fn remap_index_buffer(indices: Option<&[u32]>, vertex_count: usize, remap: &[u32]) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    match indices {
        Some(indices) => {
            result.resize(indices.len(), 0u32);
            unsafe {
                ffi::meshopt_remapIndexBuffer(
                    result.as_mut_ptr() as *mut ::std::os::raw::c_uint,
                    indices.as_ptr() as *const ::std::os::raw::c_uint,
                    indices.len(),
                    remap.as_ptr() as *const ::std::os::raw::c_uint,
                );
            }
        }
        None => {
            result.resize(vertex_count, 0u32);
            unsafe {
                ffi::meshopt_remapIndexBuffer(
                    result.as_mut_ptr() as *mut ::std::os::raw::c_uint,
                    ::std::ptr::null(),
                    0,
                    remap.as_ptr() as *const ::std::os::raw::c_uint,
                );
            }
        }
    }

    result
}

/// Generates vertex buffer from the source vertex buffer and remap table generated by generate_vertex_remap
pub fn remap_vertex_buffer<T: Clone + Default>(vertices: &[T], remap: &[u32]) -> Vec<T> {
    let mut result: Vec<T> = vec![T::default(); vertices.len()];
    unsafe {
        ffi::meshopt_remapVertexBuffer(
            result.as_mut_ptr() as *mut ::std::os::raw::c_void,
            vertices.as_ptr() as *const ::std::os::raw::c_void,
            vertices.len(),
            mem::size_of::<T>(),
            remap.as_ptr() as *const ::std::os::raw::c_uint,
        );
    }
    result
}
