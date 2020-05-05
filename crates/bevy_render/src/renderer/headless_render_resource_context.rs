use super::RenderResourceContext;
use crate::{
    render_resource::{BufferInfo, RenderResource, ResourceInfo},
    shader::Shader,
    texture::{SamplerDescriptor, TextureDescriptor},
};
use bevy_asset::{AssetStorage, Handle, HandleUntyped};
use bevy_window::{Window, WindowId};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct HeadlessRenderResourceContext {
    resource_info: Arc<RwLock<HashMap<RenderResource, ResourceInfo>>>,
    pub asset_resources: Arc<RwLock<HashMap<(HandleUntyped, usize), RenderResource>>>,
}

impl HeadlessRenderResourceContext {
    pub fn add_resource_info(&self, resource: RenderResource, resource_info: ResourceInfo) {
        self.resource_info
            .write()
            .unwrap()
            .insert(resource, resource_info);
    }
}

impl RenderResourceContext for HeadlessRenderResourceContext {
    fn create_swap_chain(&self, _window: &Window) {}
    fn next_swap_chain_texture(&self, _window_id: WindowId) -> RenderResource {
        RenderResource::new()
    }
    fn drop_swap_chain_texture(&self, _render_resource: RenderResource) {}
    fn drop_all_swap_chain_textures(&self) {}
    fn create_sampler(&self, _sampler_descriptor: &SamplerDescriptor) -> RenderResource {
        let resource = RenderResource::new();
        self.add_resource_info(resource, ResourceInfo::Sampler);
        resource
    }
    fn create_texture(&self, _texture_descriptor: &TextureDescriptor) -> RenderResource {
        let resource = RenderResource::new();
        self.add_resource_info(resource, ResourceInfo::Texture);
        resource
    }
    fn create_buffer(&self, buffer_info: BufferInfo) -> RenderResource {
        let resource = RenderResource::new();
        self.add_resource_info(resource, ResourceInfo::Buffer(buffer_info));
        resource
    }
    fn create_buffer_mapped(
        &self,
        buffer_info: BufferInfo,
        setup_data: &mut dyn FnMut(&mut [u8], &dyn RenderResourceContext),
    ) -> RenderResource {
        let mut buffer = vec![0; buffer_info.size];
        setup_data(&mut buffer, self);
        RenderResource::new()
    }
    fn create_buffer_with_data(&self, buffer_info: BufferInfo, _data: &[u8]) -> RenderResource {
        let resource = RenderResource::new();
        self.add_resource_info(resource, ResourceInfo::Buffer(buffer_info));
        resource
    }
    fn create_shader_module(
        &mut self,
        _shader_handle: Handle<Shader>,
        _shader_storage: &AssetStorage<Shader>,
    ) {
    }
    fn remove_buffer(&self, resource: RenderResource) {
        self.resource_info.write().unwrap().remove(&resource);
    }
    fn remove_texture(&self, resource: RenderResource) {
        self.resource_info.write().unwrap().remove(&resource);
    }
    fn remove_sampler(&self, resource: RenderResource) {
        self.resource_info.write().unwrap().remove(&resource);
    }
    fn get_resource_info(
        &self,
        resource: RenderResource,
        handle_info: &mut dyn FnMut(Option<&ResourceInfo>),
    ) {
        handle_info(self.resource_info.read().unwrap().get(&resource));
    }
    fn set_asset_resource_untyped(
        &self,
        handle: HandleUntyped,
        render_resource: RenderResource,
        index: usize,
    ) {
        self.asset_resources
            .write()
            .unwrap()
            .insert((handle, index), render_resource);
    }
    fn get_asset_resource_untyped(
        &self,
        handle: HandleUntyped,
        index: usize,
    ) -> Option<RenderResource> {
        self.asset_resources
            .write()
            .unwrap()
            .get(&(handle, index))
            .cloned()
    }
}