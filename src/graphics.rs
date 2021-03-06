use ImageSize;
use draw_state::DrawState;

/// Implemented by all graphics back-ends.
pub trait Graphics {
    /// The texture type associated with the back-end.
    type Texture: ImageSize;

    /// Clears background with a color.
    fn clear(&mut self, color: [f32; 4]);

    /// Renders list of 2d triangles.
    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], f: F)
        where F: FnMut(&mut FnMut(&[f32]));

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &<Self as Graphics>::Texture,
        f: F
    ) where F: FnMut(&mut FnMut(&[f32], &[f32]));
}
