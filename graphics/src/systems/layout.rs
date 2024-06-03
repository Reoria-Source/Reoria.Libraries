use crate::{AHashMap, GpuDevice};
use bytemuck::{Pod, Zeroable};
use std::{
    any::{Any, TypeId},
    rc::Rc,
};

/// Trait used to Create and Store [`wgpu::BindGroupLayout`] within a HashMap.
///
pub trait Layout: Pod + Zeroable {
    /// Creates the [`wgpu::BindGroupLayout`] to be added to the HashMap
    ///
    fn create_layout(
        &self,
        gpu_device: &mut GpuDevice,
    ) -> wgpu::BindGroupLayout;

    /// Gives a Hashable Key of the [`wgpu::BindGroupLayout`] to use to Retrieve it from the HashMap.
    ///
    fn layout_key(&self) -> (TypeId, Vec<u8>) {
        let type_id = self.type_id();
        let bytes: Vec<u8> =
            bytemuck::try_cast_slice(&[*self]).unwrap_or(&[]).to_vec();

        (type_id, bytes)
    }
}

/// [`wgpu::BindGroupLayout`] Storage within a HashMap
///
pub struct LayoutStorage {
    pub(crate) bind_group_map:
        AHashMap<(TypeId, Vec<u8>), Rc<wgpu::BindGroupLayout>>,
}

impl LayoutStorage {
    /// Creates a new [`LayoutStorage`] with Default HashMap.
    ///
    pub fn new() -> Self {
        Self {
            bind_group_map: AHashMap::default(),
        }
    }

    /// Creates a new [`wgpu::BindGroupLayout`] from [`Layout`] and adds it to the internal map.
    /// Returns an Rc<wgpu::BindGroupLayout>
    pub fn create_layout<K: Layout>(
        &mut self,
        device: &mut GpuDevice,
        layout: K,
    ) -> Rc<wgpu::BindGroupLayout> {
        let key = layout.layout_key();

        let layout = self
            .bind_group_map
            .entry(key)
            .or_insert_with(|| Rc::new(layout.create_layout(device)));

        Rc::clone(layout)
    }
}

impl Default for LayoutStorage {
    fn default() -> Self {
        Self::new()
    }
}
