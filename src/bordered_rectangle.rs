use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

/// A primitive type for rectangles with transparent fill but opaque border
pub struct BorderedRectangle {
    half_size: Vec2,
    /// has to be smaller than self.half_size.[x|y]
    border_radius: f32,
}

/// By reference of https://docs.rs/bevy_mesh/0.15.3/src/bevy_mesh/primitives/dim2.rs.html#847
impl From<BorderedRectangle> for Mesh {
    fn from(value: BorderedRectangle) -> Self {
        let [hw, hh, br] = [value.half_size.x, value.half_size.y, value.border_radius];
        let positions = vec![
            [-hw, -hh, 0.],
            [-hw + br, -hh, 0.],
            [-hw + br, -hh + br, 0.],
            [-hw, -hh + br, 0.],
            //
            [hw, -hh, 0.],
            [hw, -hh + br, 0.],
            [hw - br, -hh + br, 0.],
            [hw - br, -hh, 0.],
            //
            [hw, hh, 0.],
            [hw - br, hh, 0.],
            [hw - br, hh - br, 0.],
            [hw, hh - br, 0.],
            //
            [-hw, hh, 0.],
            [-hw, hh - br, 0.],
            [-hw + br, hh - br, 0.],
            [-hw + br, hh, 0.],
        ];
        let normals = vec![[0., 0., 1.]; 16];
        // U value of V1, V2, V14 and V15
        let x_left = br / (2. * hw);
        let x_right = 1. - x_left;
        let y_top = br / (2. * hh);
        let y_bottom = 1. - y_top;

        let uvs = vec![
            [0., 1.],
            [x_left, 1.],
            [x_left, y_bottom],
            [0., y_bottom],
            //
            [1., 1.],
            [1., y_bottom],
            [x_right, y_bottom],
            [x_right, 1.],
            //
            [1., 0.],
            [x_right, 0.],
            [x_right, y_top],
            [1., y_top],
            //
            [0., 0.],
            [0., y_top],
            [x_left, y_top],
            [x_left, 0.],
        ];
        let indices = Indices::U32(vec![
            // bottom left
            3, 0, 2, //
            0, 1, 2, //
            // bottom
            2, 1, 6, //
            1, 7, 6, //
            // bottom right
            6, 7, 5, //
            7, 4, 5, //
            // right
            10, 6, 11, //
            6, 5, 11, //
                // top right
        ]);
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
    }
}
