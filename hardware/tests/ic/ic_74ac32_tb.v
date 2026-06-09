`timescale 1ns/1ps
`include "src/ic/ic_74ac32.v"

module ic_74ac283_tb;
    reg [3:0] a;
    reg [3:0] b;
    wire [3:0] y;
    wire [3:0] expected;
    assign expected = a | b;

    ic_74ac32 uut(
        .p1(a[0]),
        .p2(b[0]),
        .p3(y[0]),
        .p4(a[1]),
        .p5(b[1]),
        .p6(y[1]),
        .p8(y[2]),
        .p9(a[2]),
        .p10(b[2]),
        .p11(y[3]),
        .p12(a[3]),
        .p13(b[3])
    );

    integer i;
    integer j;
    integer k;
    initial begin
        for (i = 0; i < 16; i = i + 1) begin
            for (j = 0; j < 16; j = j + 1) begin
                for (k = 0; k < 2; k = k + 1) begin
                    a = i;
                    b = j;
                    #10
                    if (y !== expected) begin
                        $display(
                                "FAIL: %d | %d, expected %d, got %d (%b)",
                                a, b, expected, y, y);
                        $fatal(1);
                    end
                end
            end
        end
        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule