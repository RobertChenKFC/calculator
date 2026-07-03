`timescale 1ns/1ps
`include "src/ic/ic_74ac157.v"
`include "src/ic/ic_74ac157_delay.v"

module ic_74ac157_tb;
    reg sel_b;
    reg not_g;
    reg [3:0] a;
    reg [3:0] b;
    wire [3:0] y;
    wire [3:0] expected;
    assign expected = not_g ? 4'b0 : (sel_b ? b : a);

    ic_74ac157 uut(
        .p1(sel_b),
        .p2(a[0]),
        .p3(b[0]),
        .p4(y[0]),
        .p5(a[1]),
        .p6(b[1]),
        .p7(y[1]),
        .p9(y[2]),
        .p10(b[2]),
        .p11(a[2]),
        .p12(y[3]),
        .p13(b[3]),
        .p14(a[3]),
        .p15(not_g)
    );

    integer i;
    integer j;
    integer k;
    integer l;
    initial begin
        for (i = 0; i < 16; i = i + 1) begin
            for (j = 0; j < 16; j = j + 1) begin
                for (k = 0; k < 2; k = k + 1) begin
                    for (l = 0; l < 2; l = l + 1) begin
                        a = i;
                        b = j;
                        sel_b = k;
                        not_g = l;
                        #`DELAY
                        #0
                        if (y !== expected) begin
                            $display(
                                    "FAIL: mux(%d, %d, %d), enabled = %d, ",
                                    sel_b, a, b, !not_g,
                                    "expected %d, got %d (%b)",
                                    expected, y, y);
                            $fatal(1);
                        end
                    end
                end
            end
        end
        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule