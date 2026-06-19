`timescale 1ns/1ps
`include "src/common/demux_3to8.v"

module demux_3to8_tb;
    localparam ADDR_WIDTH = 3;
    localparam WIDTH = 1 << ADDR_WIDTH;

    reg[ADDR_WIDTH-1:0] addr;
    reg[WIDTH-1:0] expected;
    wire [WIDTH-1:0] actual;

    demux_3to8 uut(
        .addr(addr), .sel(actual)
    );

    integer i;
    integer j;
    initial begin
        for (i = 0; i < WIDTH; i = i + 1) begin
            addr = i;
            for (j = 0; j < WIDTH; j = j + 1) begin
                expected[j] = i !== j;
            end
            #10.1
            if (actual !== expected) begin
                $display(
                        "FAIL: demux(%d), expected %b, got %b", addr, expected,
                        actual);
                $fatal(1);
            end
        end

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule