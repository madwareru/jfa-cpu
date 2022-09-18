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

    let mut step_size: usize = WIDTH.max(HEIGHT);
    while step_size > 0 {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for idx in index_buffer.drain(..) {
            let i = idx % WIDTH;
            let j = idx / WIDTH;
            let bounds = [
                (
                    if step_size <= i { i - step_size } else { 0 },
                    if i + step_size < WIDTH { i + step_size } else { WIDTH - 1}
                ),
                (
                    if step_size <= j { j - step_size } else { 0 },
                    if j + step_size < HEIGHT { j + step_size } else { HEIGHT - 1}
                ),
            ];

            let places_to_visit = [
                (bounds[0].0, j), (bounds[0].1, j),
                (i, bounds[1].0), (i, bounds[1].1),
                (bounds[0].0, bounds[1].0), (bounds[0].1, bounds[1].0),
                (bounds[0].0, bounds[1].1), (bounds[0].1, bounds[1].1),
            ];

            let val = unsafe { *buffer.get_unchecked(idx) };

            for pos in places_to_visit {
                let ix = pos.0 + WIDTH * pos.1;
                let current = unsafe { *buffer.get_unchecked(ix) };
                if !visitor_set.contains(&(ix)) || dst(pos, val) < dst(pos, current) {
                    unsafe {
                        *buffer.get_unchecked_mut(ix) = val;
                    }
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