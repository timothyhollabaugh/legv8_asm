
#[macro_use]
extern crate nom;

#[macro_use]
pub mod bit;
pub mod instruction;
pub mod parser;
pub mod register;
pub mod shift16;
pub mod condition;
pub mod immediate;
pub mod generator;

use bit::Bit;

#[no_mangle]
pub fn parse_to_rom(assembly: &str) -> String {
    let parsed_instructions = parser::parse_lines(assembly);

    let error_lines: Vec<u32>  = parsed_instructions.iter().enumerate().filter_map(|(l, i)|{
        if i.is_err() {
            Some(l as u32)
        } else {
            None
        }
    }).collect();

    let instructions: Vec<[Bit; 32]> = parsed_instructions.into_iter().filter_map(|i| {
        if let Some(instr) = i.ok() {
            Some(instr.1)
        } else {
            None}
    }).map(|i| {
        i.into()
    }).collect();

    if error_lines.is_empty() {
        generator::generate_case_rom(instructions)
    } else {
        error_lines.into_iter().fold("".to_owned(), |mut errors, line| {
            errors.push_str(&format!("Error on line {}", line));
            errors
        })
    }
}

#[test]
fn test_parse_to_rom() {
    assert_eq!(
        parse_to_rom(
"STUR X23, [X7, 50]
ADDI X7, X7, 1
B 6
"
        ),
"module rom_case(out, address);
    output reg [31:0] out;
    input [15:0] address;
    always @ (address) begin
        case (address)
            16'd0: out = 32'b11111000000000110010000011110111;
            16'd1: out = 32'b10010001000000000000010011100111;
            16'd2: out = 32'b00010100000000000000000000000110;
            default: out = 32'hD60003E0; // BR XZR
        endcase
    end
end
"
    )
}
