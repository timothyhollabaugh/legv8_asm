use bit::Bit;

pub fn generate_binary_str(instructions: Vec<[Bit; 32]>) -> String {
    instructions
        .iter()
        .fold("".to_string(), |mut output, instr| {
            output.push_str(&instr
                .iter()
                .rev()
                .fold("".to_string(), |mut line, bit| {
                    line.push({
                        match bit {
                            &Bit::One => '1',
                            &Bit::Zero => '0',
                        }
                    });
                    line
                }));

            output.push('\n');
            output
        })
}

pub fn generate_case_rom(instructions: Vec<[Bit; 32]>) -> String {
    let mut rom: String = "".to_string();

    // Add the header
    rom.push_str("\
module rom_case(out, address);
    output reg [31:0] out;
    input [15:0] address;
    always @ (address) begin
        case (address)
");

    // Generate the cases
    let cases: String = instructions
         // Create an iterator
        .iter()
        // Include the index of the instruction from the vector
        .enumerate()
        // Build up the cases from the vector of bit arrays, starting with ""
        .fold("".to_string(), |mut output, (i, instr)| {

            // Add the line header with the address to match
            output.push_str(&format!("            16'd{}: out = 32'b", i));

            // Generate the binary instruction
            let line: String = instr
                .iter() // Create an iterator
                .rev() // Reverse it so least significant bit is last
                // Build up the binary instruction from the bit array, starting with ""
                .fold("".to_string(), |mut line, bit| {

                    // Add the bit to the line
                    line.push({
                        match bit {
                            &Bit::One => '1',
                            &Bit::Zero => '0',
                        }
                    });

                    // Return the line
                    line
                });
            // Add the binary instruction to the jump table
            output.push_str(&line);

            // Add the semicolon and newline
            output.push_str(";\n");

            // return the case
            output
        });

    // Add the cases to the rom module
    rom.push_str(&cases);

    // Add the closing bits of the function
    rom.push_str(
"            default: out = 32'hD60003E0; // BR XZR
        endcase
    end
end\n"
    );

    // Return the module
    rom
}

#[test]
fn test_generate_binary_str() {
    assert_eq!(
        generate_binary_str(vec![
            bit_array![1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            bit_array![0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
        ]),
"\
01010101010101010101010101011111
10101010101010101010101010100000
"
    );
}

#[test]
fn test_generate_case_rom() {
    assert_eq!(
        generate_case_rom(vec![
            bit_array![1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
            bit_array![1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
            bit_array![0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
        ]),
        "\
module rom_case(out, address);
    output reg [31:0] out;
    input [15:0] address;
    always @ (address) begin
        case (address)
            16'd0: out = 32'b11111000000000110010000011110111;
            16'd1: out = 32'b10010001000000000000010011100111;
            16'd2: out = 32'b00010100000000000000011111111010;
            default: out = 32'hD60003E0; // BR XZR
        endcase
    end
end
"
    )
}

