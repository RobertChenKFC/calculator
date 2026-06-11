`timescale 1ns/1ps
`include "src/ic/ic_74ac138.v"

module ic_74ac138_tb;
    localparam NUM_FLAGS = 3;
    localparam FLAG_NOT_G2A = 0;
    localparam FLAG_NOT_G2B = 1;
    localparam FLAG_G1 = 2;
    localparam ADDR_WIDTH = 3;
    localparam WIDTH = 1 << ADDR_WIDTH;

    reg[ADDR_WIDTH-1:0] addr;
    reg[NUM_FLAGS-1:0] flags;
    wire[WIDTH-1:0] actual;
    reg[WIDTH-1:0] expected;

    ic_74ac138 uut(
        .p1(addr[0]),
        .p2(addr[1]),
        .p3(addr[2]),
        .p4(flags[FLAG_NOT_G2A]),
        .p5(flags[FLAG_NOT_G2B]),
        .p6(flags[FLAG_G1]),
        .p7(actual[7]),
        .p9(actual[6]),
        .p10(actual[5]),
        .p11(actual[4]),
        .p12(actual[3]),
        .p13(actual[2]),
        .p14(actual[1]),
        .p15(actual[0])
    );

    integer i;
    integer j;
    integer k;
    initial begin
        for (i = 0; i < WIDTH; i = i + 1) begin
            addr = i;
            for (j = 0; j < (1 << NUM_FLAGS); j = j + 1) begin
                flags = j;
                for (k = 0; k < WIDTH; k = k + 1) begin
                    expected[k] = !(
                        i === k && flags[FLAG_G1] && !flags[FLAG_NOT_G2A] &&
                        !flags[FLAG_NOT_G2B]);
                end
                #10.1
                if (expected !== actual) begin
                    $display("FAIL: addr %d, flags %b, expected %b, got %b",
                             addr, flags, expected, actual);
                    $fatal(1);
                end
            end
        end
        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule