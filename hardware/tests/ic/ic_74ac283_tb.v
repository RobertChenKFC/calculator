`timescale 1ns/1ps
`include "src/ic/ic_74ac283.v"

module ic_74ac283_tb;
    reg [3:0] a;
    reg [3:0] b;
    reg c_in;
    wire [4:0] s;
    wire [4:0] expected;
    assign expected = a + b + c_in;

    ic_74ac283 uut(
        .p1(s[1]),
        .p2(b[1]),
        .p3(a[1]),
        .p4(s[0]),
        .p5(a[0]),
        .p6(b[0]),
        .p7(c_in),
        .p9(s[4]),
        .p10(s[3]),
        .p11(b[3]),
        .p12(a[3]),
        .p13(s[2]),
        .p14(a[2]),
        .p15(b[2])
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
                    c_in = k;
                    #17
                    if (s !== expected) begin
                        $display(
                                "FAIL: %d + %d + %d, expected %d, got %d (%b)",
                                a, b, c_in, expected, s, s);
                        $fatal(1);
                    end
                end
            end
        end
        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule