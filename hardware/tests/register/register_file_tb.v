`timescale 1ns/1ps
`define TEST_TIMING
`include "src/register/register_file.v"
`include "src/register/register_file_delay.v"

`define CHECK_RESULT(out, target) \
  if (out !== target) begin \
    $display( \
        "FAIL: %s, expected %d, got %d (%b)", `"out`", target, out, out); \
    $fatal(1); \
  end

module register_file_tb;
    localparam WIDTH = 8;
    localparam IDX_WIDTH = 2;
    localparam NUM_REGS = 1 << IDX_WIDTH;

    reg clk;
    reg not_we;
    reg[IDX_WIDTH-1:0] rd_idx;
    reg[IDX_WIDTH-1:0] rs0_idx;
    reg[IDX_WIDTH-1:0] rs1_idx;
    reg[WIDTH-1:0] in;
    wire[WIDTH-1:0] out0;
    wire[WIDTH-1:0] out1;

    register_file uut(
        .clk(clk),
        .not_we(not_we),
        .rd_idx(rd_idx),
        .rs0_idx(rs0_idx),
        .rs1_idx(rs1_idx),
        .in(in),
        .out0(out0),
        .out1(out1)
    );

    initial begin
        // Note: because of the way the testbench is written below (and also
        // most likely how the register file will be used in the CPU anyway),
        // we can use a delay of 29 ns and the register file will still function
        // correctly, even though the critical path of this module is 30 ns.
        // This is because the critical path is from `rd_idx` to `out0`.
        // However, in the testbench below, we run:
        //   setup all inputs -> DELAY -> change clk -> DELAY
        // which means all the inputs apart from `clk` actually has a 2*DELAY
        // headroom, while the `clk` itself only has DELAY headroom, and the
        // longest path from `clk` is 29 ns. Therefore, there is a slight room
        // for improvement here, but for now we will use the 30 ns computed by
        // the script.

        clk = 1'b0;
        not_we = 1'b0;
        rd_idx = 2'b00;
        rs0_idx = 2'b00;
        in = 12;
        #`DELAY
        clk = 1'b1;
        #`DELAY
        #0
        `CHECK_RESULT(out0, 12);
        clk = 1'b0;
        not_we = 1'b0;
        rd_idx = 2'b01;
        rs0_idx = 2'b00;
        rs1_idx = 2'b01;
        in = 34;
        #`DELAY
        clk = 1'b1;
        #`DELAY
        #0
        `CHECK_RESULT(out0, 12);
        `CHECK_RESULT(out1, 34);
        clk = 1'b0;
        not_we = 1'b0;
        rd_idx = 2'b10;
        rs0_idx = 2'b01;
        rs1_idx = 2'b10;
        in = 56;
        #`DELAY
        clk = 1'b1;
        #`DELAY
        #0
        `CHECK_RESULT(out0, 34);
        `CHECK_RESULT(out1, 56);
        clk = 1'b0;
        not_we = 1'b0;
        rd_idx = 2'b11;
        rs0_idx = 2'b10;
        rs1_idx = 2'b11;
        in = 78;
        #`DELAY
        clk = 1'b1;
        #`DELAY
        #0
        `CHECK_RESULT(out0, 56);
        `CHECK_RESULT(out1, 78);
        clk = 1'b0;
        not_we = 1'b1;
        rd_idx = 2'b00;
        rs0_idx = 2'b00;
        rs1_idx = 2'b01;
        in = 90;
        #`DELAY
        clk = 1'b1;
        #`DELAY
        #0
        `CHECK_RESULT(out0, 12);
        `CHECK_RESULT(out1, 34);
        clk = 1'b0;
        not_we = 1'b1;
        rd_idx = 2'b10;
        rs0_idx = 2'b10;
        rs1_idx = 2'b11;
        in = 87;
        #`DELAY
        clk = 1'b1;
        #`DELAY
        #0
        `CHECK_RESULT(out0, 56);
        `CHECK_RESULT(out1, 78);

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule