`timescale 1ns/1ps
`include "src/alu/adder.v"

module adder_tb;
    localparam WIDTH = 8;

    reg [WIDTH-1:0] x;
    reg [WIDTH-1:0] y;
    reg c_in;
    wire [WIDTH-1:0] actual;
    wire [WIDTH-1:0] expected;
    assign expected = x + y + c_in;

    adder uut(
        .x(x), .y(y), .c_in(c_in), .z(actual)
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
                    c_in = k;
                    #34
                    if (actual !== expected) begin
                        $display(
                                "FAIL: %d + %d + %d, expected %d, got %d (%b)",
                                x, y, c_in, expected, actual, actual);
                        $fatal(1);
                    end
                end
            end
        end

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule