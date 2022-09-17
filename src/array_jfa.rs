use std::collections::HashSet;

pub(crate) fn calc_array_jfa<const SIZE: usize>(
    point_positions: impl IntoIterator<Item = usize>,
    buffer: &mut Vec<usize>,
    index_buffer: &mut Vec<usize>,
    visitor_set: &mut HashSet<usize>
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

    let mut step_size: usize = 1;
    while visitor_set.len() < SIZE {
        index_buffer.clear();
        index_buffer.extend(visitor_set.iter().map(|it| *it));
        for i in index_buffer.drain(..) {
            if step_size <= i {
                let pos = i - step_size;
                let current = buffer[pos];
                if !visitor_set.contains(&pos) || dst(pos, i) < dst(pos, current) {
                    buffer[pos] = i;
                    visitor_set.insert(pos);
                }
            }
            if i + step_size < SIZE {
                let pos = i + step_size;
                let current = buffer[pos];
                if !visitor_set.contains(&pos) || dst(pos, i) < dst(pos, current) {
                    buffer[pos] = i;
                    visitor_set.insert(pos);
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
fn dst(lhs: usize, rhs: usize) -> usize { if lhs <= rhs { rhs - lhs } else { lhs - rhs } }