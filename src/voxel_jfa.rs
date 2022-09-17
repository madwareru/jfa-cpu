use std::collections::HashSet;

pub(crate) fn calc_voxel_jfa<const WIDTH: usize, const DEPTH: usize, const HEIGHT: usize>(
    point_positions: impl IntoIterator<Item = (usize, usize, usize)>,
    buffer: &mut Vec<(usize, usize, usize)>,
    index_buffer: &mut Vec<usize>,
    visitor_set: &mut HashSet<usize>
) {
    let idx = |x: usize, y: usize, z: usize| {
        y * WIDTH * DEPTH + z * WIDTH + x
    };

    let size: usize = WIDTH * DEPTH * HEIGHT;
    {
        buffer.clear();
        buffer.resize(size, (0, 0, 0));
        visitor_set.clear();
        for (px, py, pz) in point_positions.into_iter() {
            if idx(px, py, pz) < size {
                buffer[idx(px, py, pz)] = (px, py, pz);
                visitor_set.insert(idx(px, py, pz));
            }
        }
    }

    let mut step_size: usize = 1;
    while visitor_set.len() < size {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for id in index_buffer.drain(..) {
            let i = id % WIDTH;
            let j = id / (WIDTH * HEIGHT);
            let k = (id % (WIDTH * HEIGHT)) / WIDTH;

            if step_size <= i {
                let pos = (i - step_size, j, k);
                let current = buffer[id - step_size];
                if !visitor_set.contains(&(id - step_size)) || dst(pos, (i, j, k)) < dst(pos, current) {
                    buffer[id - step_size] = (i, j, k);
                    visitor_set.insert(id - step_size);
                }
            }

            if i + step_size < WIDTH {
                let pos = (i + step_size, j, k);
                let current = buffer[id + step_size];
                if !visitor_set.contains(&(id + step_size)) || dst(pos, (i, j, k)) < dst(pos, current) {
                    buffer[id + step_size] = (i, j, k);
                    visitor_set.insert(id + step_size);
                }
            }

            if step_size <= k {
                let pos = (i, j, k - step_size);
                let current = buffer[id - step_size * WIDTH];
                if !visitor_set.contains(&(id - step_size * WIDTH)) || dst(pos, (i, j, k)) < dst(pos, current) {
                    buffer[id - step_size * WIDTH] = (i, j, k);
                    visitor_set.insert(id - step_size * WIDTH);
                }
            }

            if k + step_size < DEPTH {
                let pos = (i, j, k + step_size);
                let current = buffer[id + step_size * WIDTH];
                if !visitor_set.contains(&(id + step_size * WIDTH)) || dst(pos, (i, j, k)) < dst(pos, current) {
                    buffer[id + step_size * WIDTH] = (i, j, k);
                    visitor_set.insert(id + step_size * WIDTH);
                }
            }

            if step_size <= j {
                let pos = (i, j - step_size, k);
                let current = buffer[id - step_size * WIDTH * HEIGHT];
                if !visitor_set.contains(&(id - step_size * WIDTH * HEIGHT)) || dst(pos, (i, j, k)) < dst(pos, current) {
                    buffer[id - step_size * WIDTH * HEIGHT] = (i, j, k);
                    visitor_set.insert(id - step_size * WIDTH * HEIGHT);
                }
            }

            if j + step_size < HEIGHT {
                let pos = (i, j + step_size, k);
                let current = buffer[id + step_size * WIDTH * HEIGHT];
                if !visitor_set.contains(&(id + step_size * WIDTH * HEIGHT)) || dst(pos, (i, j, k)) < dst(pos, current) {
                    buffer[id + step_size * WIDTH * HEIGHT] = (i, j, k);
                    visitor_set.insert(id + step_size * WIDTH * HEIGHT);
                }
            }
        }

        if usize::MAX / 2 >= step_size {
            step_size *= 2;
        } else {
            break;
        }
    }
}

#[inline(always)]
fn dst((lx, ly, lz): (usize, usize, usize), (rx, ry, rz): (usize, usize, usize)) -> f32 {
    let (dx, dy, dz) = (lx as f32 - rx as f32, ly as f32 - ry as f32, lz as f32 - rz as f32);
    dx * dx + dy * dy + dz * dz
}