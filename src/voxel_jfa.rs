use std::collections::HashSet;
use crate::Wrapping;

pub(crate) fn calc_voxel_jfa<const WIDTH: usize, const DEPTH: usize, const HEIGHT: usize>(
    point_positions: impl IntoIterator<Item = (usize, usize, usize)>,
    buffer: &mut Vec<(usize, usize, usize)>,
    index_buffer: &mut Vec<usize>,
    visitor_set: &mut HashSet<usize>,
    wrapping: Wrapping
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

    let mut step_size: usize = WIDTH.max(HEIGHT).max(DEPTH);
    while step_size > 0 {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for id in index_buffer.drain(..) {
            let x = id % WIDTH;
            let y = id / (WIDTH * DEPTH);
            let z = (id % (WIDTH * DEPTH)) / WIDTH;

            let bounds = [
                (
                    if step_size <= x { x - step_size } else {
                        match wrapping {
                            Wrapping::Clamp => 0,
                            Wrapping::Repeat => (x + WIDTH - (step_size % WIDTH)) % WIDTH
                        }
                    },
                    if x + step_size < WIDTH { x + step_size } else {
                        match wrapping {
                            Wrapping::Clamp => WIDTH - 1,
                            Wrapping::Repeat => (x + step_size) % WIDTH
                        }
                    }
                ),
                (
                    if step_size <= y { y - step_size } else {
                        match wrapping {
                            Wrapping::Clamp => 0,
                            Wrapping::Repeat => (y + HEIGHT - (step_size % HEIGHT)) % HEIGHT
                        }
                    },
                    if y + step_size < HEIGHT { y + step_size } else {
                        match wrapping {
                            Wrapping::Clamp => HEIGHT - 1,
                            Wrapping::Repeat => (y + step_size) % HEIGHT
                        }
                    }
                ),
                (
                    if step_size <= z { z - step_size } else {
                        match wrapping {
                            Wrapping::Clamp => 0,
                            Wrapping::Repeat => (z + DEPTH - (step_size % DEPTH)) % DEPTH
                        }
                    },
                    if z + step_size < DEPTH { z + step_size } else {
                        match wrapping {
                            Wrapping::Clamp => DEPTH - 1,
                            Wrapping::Repeat => (z + step_size) % DEPTH
                        }
                    }
                ),
            ];

            let val = unsafe { *buffer.get_unchecked(idx(x, y, z)) };

            for xx in -1..=1 {
                for yy in -1..=1 {
                    for zz in -1..=1 {
                        if xx + yy + zz == 0 {
                            continue;
                        }
                        let pos = (
                            match xx {
                                -1 => bounds[0].0,
                                1 => bounds[0].1,
                                _ => x
                            },
                            match yy {
                                -1 => bounds[1].0,
                                1 => bounds[1].1,
                                _ => y
                            },
                            match zz {
                                -1 => bounds[2].0,
                                1 => bounds[2].1,
                                _ => z
                            }
                        );

                        let ix = idx(pos.0, pos.1, pos.2);
                        let current = unsafe { *buffer.get_unchecked(ix) };
                        if !visitor_set.contains(&(ix)) || dst(pos, val) < dst(pos, current) {
                            unsafe {
                                *buffer.get_unchecked_mut(ix) = val;
                            }
                            visitor_set.insert(ix);
                        }
                    }
                }
            }
        }
        step_size /= 2;
    }
}

#[inline(always)]
fn dst((lx, ly, lz): (usize, usize, usize), (rx, ry, rz): (usize, usize, usize)) -> f32 {
    let (dx, dy, dz) = (lx as f32 - rx as f32, ly as f32 - ry as f32, lz as f32 - rz as f32);
    dx * dx + dy * dy + dz * dz
}