//ShiftRows
//For every "row" (0-3, 4-7, 8-11, 12-15), shift the content over.
//

pub fn shift_rows(&mut state: State) {
    for i in (0..4) {
        let row_start = i * 4;
        let row = &mut state.state_box[row_start..row_start + 4];
        row.rotate_right(i);
    }
}
