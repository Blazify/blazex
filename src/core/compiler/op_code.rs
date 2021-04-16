#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    OpConstant(u16),
    OpPop,
    OpAdd,
    OpSub,
    OpPlus,
    OpMinus,
}

pub fn convert_u16_to_two_u8s(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert_two_u8s_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_u16_to_two_u8s(data));
    output
}

pub fn make_op(op: OpCode) -> Vec<u8> {
    match op {
        OpCode::OpConstant(arg) => make_three_byte_op(0x01, arg),
        OpCode::OpPop => vec![0x02],
        OpCode::OpAdd => vec![0x03],
        OpCode::OpSub => vec![0x04],
        OpCode::OpPlus => vec![0x0A],
        OpCode::OpMinus => vec![0x0B],
    }
}
