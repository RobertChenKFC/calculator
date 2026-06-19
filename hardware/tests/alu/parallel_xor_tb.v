`timescale 1ns/1ps
`include "src/alu/parallel_xor.v"
`include "src/alu/parallel_xor_delay.v"

module parallel_xor_tb;
    localparam WIDTH = 8;

    reg [WIDTH-1:0] x;
    reg [WIDTH-1:0] y;
    wire [WIDTH-1:0] actual;
    wire [WIDTH-1:0] expected;
    assign expected = x ^ y;

    parallel_xor uut(
        .x(x), .y(y), .z(actual)
    );

    integer i;
    integer j;
    initial begin
        for (i = 0; i < 256; i = i + 1) begin
            for (j = 0; j < 256; j = j + 1) begin
                x = i;
                y = j;
                #`DELAY
                #0
                if (actual !== expected) begin
                    $display(
                            "FAIL: %d ^ %d, expected %d, got %d (%b)",
                            x, y, expected, actual, actual);
                    $fatal(1);
                end
            end
        end

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule