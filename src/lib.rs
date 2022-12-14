use std::collections::HashSet;

mod array_jfa;
mod matrix_jfa;
mod voxel_jfa;

#[derive(Copy, Clone)]
pub enum Wrapping {
    Clamp,
    Repeat
}

pub struct ArrayJfa {
    visitor_set: HashSet<usize>,
    buffer: Vec<usize>,
    index_buffer: Vec<usize>
}

pub struct MatrixJfa {
    visitor_set: HashSet<usize>,
    buffer: Vec<(usize, usize)>,
    index_buffer: Vec<usize>
}

pub struct VoxelJfa {
    visitor_set: HashSet<usize>,
    buffer: Vec<(usize, usize, usize)>,
    index_buffer: Vec<usize>
}

impl ArrayJfa {
    pub fn new() -> Self {
        Self {
            visitor_set: HashSet::new(),
            buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

    pub fn calc<const SIZE: usize>(
        &mut self,
        point_positions: impl IntoIterator<Item = usize>,
        wrapping: Wrapping
    ) -> &[usize] {
        array_jfa::calc_array_jfa::<SIZE>(
            point_positions,
            &mut self.buffer,
            &mut self.index_buffer,
            &mut self.visitor_set,
            wrapping
        );
        &self.buffer
    }

    pub fn calc_to_buffer<const SIZE: usize>(
        &mut self,
        point_positions: impl IntoIterator<Item = usize>,
        buffer: &mut Vec<usize>,
        wrapping: Wrapping
    ) {
        array_jfa::calc_array_jfa::<SIZE>(
            point_positions,
            buffer,
            &mut self.index_buffer,
            &mut self.visitor_set,
            wrapping
        );
    }
}

impl MatrixJfa {
    pub fn new() -> Self {
        Self {
            visitor_set: HashSet::new(),
            buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

    pub fn calc<const WIDTH: usize, const HEIGHT: usize>(
        &mut self,
        point_positions: impl IntoIterator<Item = (usize, usize)>,
        wrapping: Wrapping
    ) -> &[(usize, usize)] {
        matrix_jfa::calc_matrix_jfa::<WIDTH, HEIGHT>(
            point_positions,
            &mut self.buffer,
            &mut self.index_buffer,
            &mut self.visitor_set,
            wrapping
        );
        &self.buffer
    }

    pub fn calc_to_buffer<const WIDTH: usize, const HEIGHT: usize>(
        &mut self, point_positions: impl IntoIterator<Item = (usize, usize)>,
        buffer: &mut Vec<(usize, usize)>,
        wrapping: Wrapping
    ) {
        matrix_jfa::calc_matrix_jfa::<WIDTH, HEIGHT>(
            point_positions,
            buffer,
            &mut self.index_buffer,
            &mut self.visitor_set,
            wrapping
        );
    }
}

impl VoxelJfa {
    pub fn new() -> Self {
        Self {
            visitor_set: HashSet::new(),
            buffer: Vec::new(),
            index_buffer: Vec::new()
        }
    }

    pub fn calc<const WIDTH: usize, const DEPTH: usize, const HEIGHT: usize>(
        &mut self,
        point_positions: impl IntoIterator<Item = (usize, usize, usize)>,
        wrapping: Wrapping
    ) -> &[(usize, usize, usize)] {
        voxel_jfa::calc_voxel_jfa::<WIDTH, DEPTH, HEIGHT>(
            point_positions,
            &mut self.buffer,
            &mut self.index_buffer,
            &mut self.visitor_set,
            wrapping
        );
        &self.buffer
    }

    pub fn calc_to_buffer<const WIDTH: usize, const DEPTH: usize, const HEIGHT: usize>(
        &mut self, point_positions: impl IntoIterator<Item = (usize, usize, usize)>,
        buffer: &mut Vec<(usize, usize, usize)>,
        wrapping: Wrapping
    ) {
        voxel_jfa::calc_voxel_jfa::<WIDTH, DEPTH, HEIGHT>(
            point_positions,
            buffer,
            &mut self.index_buffer,
            &mut self.visitor_set,
            wrapping
        );
    }
}