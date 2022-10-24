pub fn rotate_right<T, const LEN: usize>(input: &mut [[T; LEN]; LEN], length: usize)
where
    T: Default + Copy,
{
    let mut output = [[T::default(); LEN]; LEN];

    for (y, row) in input.into_iter().enumerate() {
        if y >= length {
            break;
        }

        for (x, cell) in row.into_iter().enumerate() {
            if x >= length {
                break;
            }

            let (x, y) = (length - 1 - y, x);

            output[y][x] = cell.to_owned();
        }
    }

    *input = output;
}

pub fn rotate_left<T, const LEN: usize>(input: &mut [[T; LEN]; LEN], length: usize)
where
    T: Default + Copy,
{
    let mut output = [[T::default(); LEN]; LEN];

    for (y, row) in input.into_iter().enumerate() {
        if y >= length {
            break;
        }

        for (x, cell) in row.into_iter().enumerate() {
            if x >= length {
                break;
            }

            let (x, y) = (y, length - 1 - x);

            output[y][x] = cell.to_owned();
        }
    }

    *input = output;
}

//SRS TABLE: IF ROTATION FAILS, TRY THESE
pub const KICK_INDEX_3BY3: [[[i64; 2];4];8] = [
    [[-1, 0], [-1, 1], [0, -2], [-1, -2]], //01 -> kickIndexJLTSZ[0][i] for i in (0,1,2,3)
    [[1, 0], [1, -1], [0, 2], [1, 2]],     //12
    [[1, 0], [1, 1], [0, -2], [1, -2]],    //23
    [[-1, 0], [-1, -1], [0, 2], [-1, 2]],  //30
    [[1, 0], [1, 1], [0, -2], [1, -2]],    //03
    [[1, 0], [1, -1], [0, 2], [1, 2]],     //10
    [[-1, 0], [-1, 1], [0, -2], [-1, -2]], //21
    [[-1, 0], [-1, -1], [0, 2], [-1, 2]],  //32
];

pub const KICK_INDEX_I: [[[i64; 2];4];8] = [
    [[-2, 0], [1, 0], [-2, -1], [1, 2]], //01 -> kickIndexI[0][i] for i in (0,1,2,3)
    [[-1, 0], [2, 0], [-1, 2], [2, -1]], //12
    [[2, 0], [-1, 0], [2, 1], [-1, -2]], //23
    [[1, 0], [-2, 0], [1, -2], [-2, 1]], //30
    [[-1, 0], [2, 0], [-1, 2], [2, -1]], //03
    [[2, 0], [-1, 0], [2, 1], [-1, -2]], //10
    [[1, 0], [-2, 0], [1, -2], [-2, 1]], //21
    [[-2, 0], [1, 0], [-2, -1], [1, 2]], //32
];

