`include "src/alu/adder.v"
`include "src/alu/mux_2to1.v"
`include "src/alu/parallel_and.v"
`include "src/alu/parallel_or.v"
`include "src/alu/parallel_xor.v"

module alu #(
    parameter WIDTH=8
) (
    input[WIDTH-1:0] rs0,
    input[WIDTH-1:0] rs1,
    input[WIDTH-1:0] imm,
    input mask_rs0,
    input invert_rs1,
    input select_imm,
    input add_cin,
    input select_add_logic,
    input select_and_or,
    output[WIDTH-1:0] out
);
    wire[WIDTH-1:0] x;
    wire[WIDTH-1:0] rs1_xored;
    wire[WIDTH-1:0] y;
    wire[WIDTH-1:0] sum_result;
    wire[WIDTH-1:0] and_result;
    wire[WIDTH-1:0] or_result;
    wire[WIDTH-1:0] z_logic;

    parallel_and and_rs0(
        .x({WIDTH{mask_rs0}}),
        .y(rs0),
        .z(x)
    );

    parallel_xor xor_rs1(
        .x({WIDTH{invert_rs1}}),
        .y(rs1),
        .z(rs1_xored)
    );

    adder add_x_y(
        .x(x),
        .y(y),
        .c_in(add_cin),
        .z(sum_result)
    );

    parallel_and and_x_y(
        .x(x),
        .y(y),
        .z(and_result)
    );

    parallel_or or_x_y(
        .x(x),
        .y(y),
        .z(or_result)
    );

    mux_2to1 sel_y(
        .sel(select_imm),
        .x(rs1_xored),
        .y(imm),
        .z(y)
    );

    mux_2to1 sel_z_logic(
        .sel(select_and_or),
        .x(and_result),
        .y(or_result),
        .z(z_logic)
    );

    mux_2to1 sel_out(
        .sel(select_add_logic),
        .x(sum_result),
        .y(z_logic),
        .z(out)
    );
endmodule