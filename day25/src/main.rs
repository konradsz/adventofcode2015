fn get_code_number(row: usize, column: usize) -> usize {
    let mut position = (1, 1);
    let mut counter = 1;

    loop {
        counter += 1;
        if position.0 == 1 {
            position.0 = position.1 + 1;
            position.1 = 1;
        } else {
            position.0 -= 1;
            position.1 += 1;
        }

        if position == (row, column) {
            break;
        }
    }

    counter
}

fn main() {
    let code_number = get_code_number(2_978, 3_083);

    let mut code: usize = 20_151_125;
    for _ in 1..code_number {
        code *= 252_533;
        code %= 33_554_393;
    }

    assert_eq!(2_650_453, code);
}
