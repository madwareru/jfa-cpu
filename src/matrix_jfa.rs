use std::collections::HashSet;
use crate::Wrapping;

pub(crate) fn calc_matrix_jfa<const WIDTH: usize, const HEIGHT: usize>(
    point_positions: impl IntoIterator<Item = (usize, usize)>,
    buffer: &mut Vec<(usize, usize)>,
    index_buffer: &mut Vec<usize>,
    visitor_set: &mut HashSet<usize>,
    wrapping: Wrapping
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
                    if step_size <= i { i - step_size } else {
                        match wrapping {
                            Wrapping::Clamp => 0,
                            Wrapping::Repeat => (i + WIDTH - (step_size % WIDTH)) % WIDTH
                        }
                    },
                    if i + step_size < WIDTH { i + step_size } else {
                        match wrapping {
                            Wrapping::Clamp => WIDTH - 1,
                            Wrapping::Repeat => (i + step_size) % WIDTH
                        }
                    }
                ),
                (
                    if step_size <= j { j - step_size } else {
                        match wrapping {
                            Wrapping::Clamp => 0,
                            Wrapping::Repeat => (j + HEIGHT - (step_size % HEIGHT)) % HEIGHT
                        }
                    },
                    if j + step_size < HEIGHT { j + step_size } else {
                        match wrapping {
                            Wrapping::Clamp => HEIGHT - 1,
                            Wrapping::Repeat => (j + step_size) % HEIGHT
                        }
                    }
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

                if !visitor_set.contains(&(ix)) {
                    unsafe {
                        *buffer.get_unchecked_mut(ix) = val;
                    }
                    visitor_set.insert(ix);
                } else {
                    let dst_to_val = match wrapping {
                        Wrapping::Clamp => dst(pos, val),
                        Wrapping::Repeat => wrapping_dst::<WIDTH, HEIGHT>(
                            pos,
                            val
                        )
                    };

                    let dst_to_current = match wrapping {
                        Wrapping::Clamp => dst(pos, current),
                        Wrapping::Repeat => wrapping_dst::<WIDTH, HEIGHT>(
                            pos,
                            current
                        )
                    };

                    if dst_to_val < dst_to_current {
                        unsafe {
                            *buffer.get_unchecked_mut(ix) = val;
                        }
                    }
                }
            }
        }
        step_size /= 2;
    }
}

#[inline(always)]
fn dst((lx, ly): (usize, usize), (rx, ry): (usize, usize)) -> f32 {
    let (dx, dy) = (lx as f32 - rx as f32, ly as f32 - ry as f32);
    dx * dx + dy * dy
}

#[inline(always)]
fn wrapping_dst<const W: usize, const H: usize>((lx, ly): (usize, usize), (rx, ry): (usize, usize)) -> f32 {
    let (dx, dy) = (lx as f32 - rx as f32, ly as f32 - ry as f32);
    let dx = dx.min(W as f32 - dx);
    let dy = dy.min(H as f32 - dy);
    dx * dx + dy * dy
}