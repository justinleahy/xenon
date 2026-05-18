# Renderer Resource Ownership

This document explains how renderer resources are owned, how long they live, and what rules to follow when adding new rendering behavior.

The intended reader is an engine contributor adding or changing renderer resources. After reading this, they should be able to add a buffer, texture, material field, or draw-path change without breaking GPU lifetime, ownership, or asset boundaries.

## Ownership Model

The renderer owns all live GPU resources needed to draw the current frame. That includes the GPU instance, surface, adapter, device, queue, surface configuration, render pipeline, vertex and index buffers, instance buffer, texture bind-group layout, and the default texture.

Callers do not own GPU resources directly. They submit plain render data through a render scene. The render scene is a short-lived view of what should be drawn this frame, not a storage layer and not a resource cache.

The window or platform layer owns the window handle. The renderer owns the surface created from that handle for as long as the renderer exists. This is why the renderer carries a window lifetime parameter.

## Resource Lifetimes

Renderer-created GPU resources live until the renderer is dropped or until a future renderer API explicitly replaces them.

Texture resources own their GPU texture, view, sampler, and bind group. The renderer can create texture resources because it owns the device, queue, and texture bind-group layout. CPU texture data is only staging input; it is not retained by the renderer after upload.

The default texture is a one-pixel white texture owned by the renderer. It keeps color-only sprites on the same shader path as textured sprites: sampling white and multiplying by material color produces the material color.

Per-frame sprite data is copied into the renderer-owned instance buffer during rendering. The render scene and its sprite slice only need to stay alive for the render call.

## Materials

A material is render data, not gameplay state. The current material representation contains a base color. During instance generation, the renderer copies that base color into the instance buffer and the shader multiplies it with the sampled texture.

Gameplay and scene code may keep higher-level sprite data. The render bridge is responsible for converting that data into render sprites with materials.

Future material fields should keep the same split:

- Value data such as tint, alpha, flags, or scalar parameters belongs directly in the material.
- Asset identity should use stable handles or IDs, not raw paths.
- GPU objects such as textures, views, samplers, bind groups, buffers, and pipelines belong to renderer-owned resources, not to gameplay components.

## Scene Submission

A render scene contains a camera and a borrowed list of render sprites. It does not create, cache, or destroy resources.

The renderer currently batches sprites into one instanced quad draw. Every submitted sprite uses the same pipeline and, for a given render call, the same texture bind group. Color-only rendering uses the default white texture.

If a future change adds per-material textures, the renderer should group or sort draw items by the GPU state they require rather than binding a different texture for every sprite without a batching plan.

## Surface And Resize Behavior

The renderer owns the surface configuration. Resize requests with a zero width or height are ignored, because minimized or hidden windows can temporarily report invalid drawable sizes.

When the drawable size changes, the renderer updates the surface configuration and reconfigures the surface. If the surface is lost or outdated while acquiring a frame, the renderer reconfigures it and skips that frame. If the surface is occluded or times out, the renderer treats the frame as non-fatal and returns without drawing.

Surface validation errors are returned to the caller because they indicate a real renderer contract violation.

## Extension Rules

When adding renderer functionality, follow these rules:

- Keep GPU resource ownership inside the renderer or renderer-owned resource types.
- Keep render scene inputs plain, short-lived data that can be rebuilt each frame.
- Do not store raw filesystem paths in render data. Resolve paths through the asset layer before renderer upload.
- Do not let gameplay components own GPU objects.
- Prefer stable asset handles or IDs at system boundaries.
- Reuse the existing device, queue, bind-group layouts, and pipeline ownership pattern.
- Add tests for pure layout, conversion, and capacity rules. Use runtime rendering checks only when behavior requires an actual surface.

## Current Limits

The renderer is intentionally small. It has one sprite pipeline, one instance buffer, one texture layout, and one texture bind group per render call. This is enough for the current sandbox and keeps ownership clear while the asset pipeline matures.

Broader material features, multiple pipelines, render passes, and texture-atlas or bindless strategies should be added only when the sandbox produces a concrete need for them.
