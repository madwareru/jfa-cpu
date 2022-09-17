use std::collections::HashSet;

pub(crate) fn calc_matrix_jfa<const WIDTH: usize, const HEIGHT: usize>(
    point_positions: impl IntoIterator<Item = (usize, usize)>,
    buffer: &mut Vec<(usize, usize)>,
    index_buffer: &mut Vec<usize>,
    visitor_set: &mut HashSet<usize>
) {
    {
        buffer.clear();
        buffer.resize(WIDTH * HEIGHT, (0, 0));
        visitor_set.clear();
        for (px, py) in point_positions.into_iter() {
            let idx = py * WIDTH + px;
            if idx < WIDTH * HEIGHT {
                buffer[idx] = (px, py);
                visitor_set.insert(idx);
            }
        }
    }

    let mut step_size: usize = 1;
    while visitor_set.len() < WIDTH * HEIGHT {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for idx in index_buffer.drain(..) {
            let i = idx % WIDTH;
            let j = idx / WIDTH;
            if step_size <= i {
                let pos = (i - step_size, j);
                let current = buffer[idx - step_size];
                if !visitor_set.contains(&(idx - step_size)) || dst(pos, (i, j)) < dst(pos, current) {
                    buffer[idx - step_size] = (i, j);
                    visitor_set.insert(idx - step_size);
                }
            }
            if i + step_size < WIDTH {
                let pos = (i + step_size, j);
                let current = buffer[idx + step_size];
                if !visitor_set.contains(&(idx + step_size)) || dst(pos, (i, j)) < dst(pos, current) {
                    buffer[idx + step_size] = (i, j);
                    visitor_set.insert(idx + step_size);
                }
            }
            if step_size <= j {
                let pos = (i, j - step_size);
                let current = buffer[idx - step_size * WIDTH];
                if !visitor_set.contains(&(idx - step_size * WIDTH)) || dst(pos, (i, j)) < dst(pos, current) {
                    buffer[idx - step_size * WIDTH] = (i, j);
                    visitor_set.insert(idx - step_size * WIDTH);
                }
            }
            if j + step_size < HEIGHT {
                let pos = (i, j + step_size);
                let current = buffer[idx + step_size * WIDTH];
                if !visitor_set.contains(&(idx + step_size * WIDTH)) || dst(pos, (i, j)) < dst(pos, current) {
                    buffer[idx + step_size * WIDTH] = (i, j);
                    visitor_set.insert(idx + step_size * WIDTH);
                }
            }
        }

        step_size *= 2;
    }
}

#[inline(always)]
fn dst((lx, ly): (usize, usize), (rx, ry): (usize, usize)) -> f32 {
    let (dx, dy) = (lx as f32 - rx as f32, ly as f32 - ry as f32);
    dx * dx + dy * dy
}