#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("failed to create render surface")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),

    #[error("failed to find compatible GPU adapter")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),

    #[error("failed to create device")]
    CreateDevice(#[from] wgpu::RequestDeviceError),

    #[error("surface is not supported by the selected adapter")]
    UnsupportedSurface,

    #[error("surface validation failed")]
    SurfaceValidation,
}
