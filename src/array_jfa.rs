use std::collections::HashSet;
use crate::Wrapping;

pub(crate) fn calc_array_jfa<const SIZE: usize>(
    point_positions: impl IntoIterator<Item = usize>,
    buffer: &mut Vec<usize>,
    index_buffer: &mut Vec<usize>,
    visitor_set: &mut HashSet<usize>,
    wrapping: Wrapping
) {
    {
        buffer.clear();
        buffer.resize(SIZE, 0);
        visitor_set.clear();
        for p in point_positions.into_iter().filter(|&it| it < SIZE) {
            buffer[p] = p;
            visitor_set.insert(p);
        }
    }

    let mut step_size: usize = SIZE;
    while step_size > 0 {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for i in index_buffer.drain(..) {
            let bounds = [
                if step_size <= i { i - step_size } else {
                    match wrapping {
                        Wrapping::Clamp => 0,
                        Wrapping::Repeat => (i + SIZE - (step_size % SIZE)) % SIZE
                    }
                },
                if i + step_size < SIZE { i + step_size } else {
                    match wrapping {
                        Wrapping::Clamp => SIZE - 1,
                        Wrapping::Repeat => (i + step_size) % SIZE
                    }
                }
            ];

            let val = unsafe { *buffer.get_unchecked(i) };

            for ix in bounds {
                let current = unsafe { *buffer.get_unchecked(ix) };
                if !visitor_set.contains(&(ix)) || dst(ix, val) < dst(ix, current) {
                    unsafe {
                        *buffer.get_unchecked_mut(ix) = val;
                    }
                    visitor_set.insert(ix);
                }
            }
        }
        step_size /= 2;
    }
}

#[inline(always)]
fn dst(lhs: usize, rhs: usize) -> usize { if lhs <= rhs { rhs - lhs } else { lhs - rhs } }