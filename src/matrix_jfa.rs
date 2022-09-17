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

    let mut step_size: usize = 2048;
    while step_size > 0 {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for idx in index_buffer.drain(..) {
            let i = idx % WIDTH;
            let j = idx / WIDTH;
            if step_size <= i {
                let pos = (i - step_size, j);
                let ix = idx - step_size;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            } else {
                let pos = (0, j);
                let ix = pos.1 * WIDTH;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            }
            if i + step_size < WIDTH {
                let pos = (i + step_size, j);
                let ix = idx + step_size;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            } else {
                let pos = (WIDTH-1, j);
                let ix = pos.0 + pos.1 * WIDTH;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            }
            if step_size <= j {
                let pos = (i, j - step_size);
                let ix = idx - step_size * WIDTH;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            } else {
                let pos = (i, 0);
                let ix = pos.0;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            }
            if j + step_size < HEIGHT {
                let pos = (i, j + step_size);
                let ix = idx + step_size * WIDTH;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            } else {
                let pos = (i, HEIGHT - 1);
                let ix = pos.0 + pos.1 * WIDTH;
                let current = buffer[ix];
                if !visitor_set.contains(&(ix)) || dst(pos, buffer[idx]) < dst(pos, current) {
                    buffer[ix] = buffer[idx];
                    visitor_set.insert(ix);
                }
            }
        }

        step_size /= 2;
    }
    println!("visitor set filled by {} items", visitor_set.len());
}

#[inline(always)]
fn dst((lx, ly): (usize, usize), (rx, ry): (usize, usize)) -> f32 {
    let (dx, dy) = (lx as f32 - rx as f32, ly as f32 - ry as f32);
    dx * dx + dy * dy
}