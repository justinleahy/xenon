pub struct TextureResource {
    pub(crate) _texture: wgpu::Texture,
    pub(crate) _view: wgpu::TextureView,
    pub(crate) _sampler: wgpu::Sampler,
    pub(crate) bind_group: wgpu::BindGroup,
}
