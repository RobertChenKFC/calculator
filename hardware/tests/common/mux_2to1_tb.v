`timescale 1ns/1ps
`include "src/common/mux_2to1.v"

module mux_2to1_tb;
    localparam WIDTH = 8;

    reg sel;
    reg [WIDTH-1:0] x;
    reg [WIDTH-1:0] y;
    wire [WIDTH-1:0] actual;
    wire [WIDTH-1:0] expected;
    assign expected = sel ? y : x;

    mux_2to1 uut(
        .sel(sel), .x(x), .y(y), .z(actual)
    );

    integer i;
    integer j;
    integer k;
    initial begin
        for (i = 0; i < 256; i = i + 1) begin
            for (j = 0; j < 256; j = j + 1) begin
                for (k = 0; k < 2; k = k + 1) begin
                    x = i;
                    y = j;
                    #13.3
                    if (actual !== expected) begin
                        $display(
                                "FAIL: mux(%d, %d, %d), ",
                                sel, x, y,
                                "expected %d, got %d (%b)",
                                expected, actual, actual);
                        $fatal(1);
                    end
                end
            end
        end

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule