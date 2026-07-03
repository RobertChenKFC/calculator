`timescale 1ns/1ps
`include "src/alu/alu.v"
`include "src/alu/alu_delay.v"

`define WIDTH 8
`define IS_VALID(expected, actual) \
    ((expected) === (actual) && \
     (expected) !== {`WIDTH{1'bx}} && \
     (expected) !== {`WIDTH{1'bx}})

module alu_tb;
    localparam NUM_RAND_INPUTS = 1024;
    localparam WIDTH = `WIDTH;
    localparam NUM_FLAGS = 6;
    localparam MASK_RS0 = 0;
    localparam INVERT_RS1 = 1;
    localparam SELECT_IMM = 2;
    localparam ADD_CIN = 3;
    localparam SELECT_ADD_LOGIC = 4;
    localparam SELECT_AND_OR = 5;

    reg [31:0] seed;
    reg [31:0] dummy;
    reg [WIDTH-1:0] rs0;
    reg [WIDTH-1:0] rs1;
    reg [WIDTH-1:0] imm;
    reg [NUM_FLAGS-1:0] flags;
    wire [WIDTH-1:0] actual;
    reg [WIDTH-1:0] expected;

    alu uut(
        .rs0(rs0),
        .rs1(rs1),
        .imm(imm),
        .mask_rs0(flags[MASK_RS0]),
        .invert_rs1(flags[INVERT_RS1]),
        .select_imm(flags[SELECT_IMM]),
        .add_cin(flags[ADD_CIN]),
        .select_add_logic(flags[SELECT_ADD_LOGIC]),
        .select_and_or(flags[SELECT_AND_OR]),
        .out(actual)
    );


    integer i;
    integer j;
    initial begin
        seed = 0;
        dummy = $random(seed);

        // Test 1: imm
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b0;
                flags[SELECT_IMM] = 1'b1;
                flags[ADD_CIN] = 1'b0;
                flags[SELECT_ADD_LOGIC] = 1'b0;
                expected = imm;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: imm, expected %d, got %d (%b)",
                            expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 2: rs0 + rs1
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[INVERT_RS1] = 1'b0;
                flags[SELECT_IMM] = 1'b0;
                flags[ADD_CIN] = 1'b0;
                flags[SELECT_ADD_LOGIC] = 1'b0;
                expected = rs0 + rs1;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d + %d, expected %d, got %d (%b)",
                            rs0, rs1, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 3: rs0 + imm
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[SELECT_IMM] = 1'b1;
                flags[ADD_CIN] = 1'b0;
                flags[SELECT_ADD_LOGIC] = 1'b0;
                expected = rs0 + imm;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d + %d, expected %d, got %d (%b)",
                            rs0, imm, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 4: rs0 - rs1
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[INVERT_RS1] = 1'b1;
                flags[SELECT_IMM] = 1'b0;
                flags[ADD_CIN] = 1'b1;
                flags[SELECT_ADD_LOGIC] = 1'b0;
                expected = rs0 - rs1;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d + %d, expected %d, got %d (%b)",
                            rs0, imm, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 5: ~rs1
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b0;
                flags[INVERT_RS1] = 1'b1;
                flags[SELECT_IMM] = 1'b0;
                flags[ADD_CIN] = 1'b0;
                flags[SELECT_ADD_LOGIC] = 1'b0;
                expected = ~rs1;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: ~%d, expected %d, got %d (%b)",
                            rs1, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 6: rs0 & rs1
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[INVERT_RS1] = 1'b0;
                flags[SELECT_IMM] = 1'b0;
                flags[SELECT_ADD_LOGIC] = 1'b1;
                flags[SELECT_AND_OR] = 1'b0;
                expected = rs0 & rs1;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d & %d, expected %d, got %d (%b)",
                            rs0, rs1, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 7: rs0 & imm
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[SELECT_IMM] = 1'b1;
                flags[SELECT_ADD_LOGIC] = 1'b1;
                flags[SELECT_AND_OR] = 1'b0;
                expected = rs0 & imm;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d & %d, expected %d, got %d (%b)",
                            rs0, imm, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 8: rs0 | rs1
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[INVERT_RS1] = 1'b0;
                flags[SELECT_IMM] = 1'b0;
                flags[SELECT_ADD_LOGIC] = 1'b1;
                flags[SELECT_AND_OR] = 1'b1;
                expected = rs0 | rs1;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d | %d, expected %d, got %d (%b)",
                            rs0, rs1, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        // Test 9: rs0 & imm
        for (i = 0; i < NUM_RAND_INPUTS; i = i + 1) begin
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                rs0 = $random & {WIDTH{1'b1}};
                rs1 = $random & {WIDTH{1'b1}};
                imm = $random & {WIDTH{1'b1}};
                flags = j;
                flags[MASK_RS0] = 1'b1;
                flags[SELECT_IMM] = 1'b1;
                flags[SELECT_ADD_LOGIC] = 1'b1;
                flags[SELECT_AND_OR] = 1'b1;
                expected = rs0 | imm;
                #`DELAY
                #0
                if (!`IS_VALID(expected, actual)) begin
                    $display(
                            "FAIL: %d | %d, expected %d, got %d (%b)",
                            rs0, imm, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule