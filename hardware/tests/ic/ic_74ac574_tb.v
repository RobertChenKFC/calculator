`timescale 1ns/1ps
`define TEST_TIMING
`include "src/ic/ic_74ac574.v"

`define CHECK_TIMING(uut, field, expected) \
  if (uut.field !== (expected)) begin \
    $display("Expected timing check %s to be %d, got %d instead", \
             `"field`", (expected), uut.field); \
    $fatal(1); \
  end

module ic_74ac574_tb;
    reg not_oe;
    reg clk;
    reg[7:0] in;
    wire[7:0] out;
    reg[7:0] expected;

    ic_74ac574 uut(
        .p1(not_oe),
        .p2(in[0]),
        .p3(in[1]),
        .p4(in[2]),
        .p5(in[3]),
        .p6(in[4]),
        .p7(in[5]),
        .p8(in[6]),
        .p9(in[7]),
        .p11(clk),
        .p12(out[7]),
        .p13(out[6]),
        .p14(out[5]),
        .p15(out[4]),
        .p16(out[3]),
        .p17(out[2]),
        .p18(out[1]),
        .p19(out[0])
    );

    integer i;
    integer j;

    initial begin
        for (i = 0; i < 256; i = i + 1) begin
            for (j = 0; j < 2; j = j + 1) begin
                not_oe = j;
                clk = 0;
                #3.0 // 3.0 + 2.1 = 5.1 > 5.0 pulse duration
                in = i;
                #2.1 // 2.1 > 2.0 setup time
                clk = 1;
                #1.6 // 1.6 > 1.5 hold time
                #3.5 // 1.6 + 3.5 = 5.1 > 5.0 pulse duration
                clk = 0;
                #6 // 1.6 + 3.5 + 6 = 11.1 > 11 propagation delay
                expected = not_oe ? 8'bz : in;
                if (out !== expected) begin
                    $display("Expected %d, got %d (%b)", expected, out, out);
                    $fatal(1);
                end
            end
        end
        `CHECK_TIMING(uut, violate_setup_time, 1'b0);
        `CHECK_TIMING(uut, violate_pulse_duration, 1'b0);
        `CHECK_TIMING(uut, violate_hold_time, 1'b0);

        // TODO: continue here: write the tests that violate the setup, hold and
        // pulse width checks, and check that the violations are reported.

        not_oe = 1'b0;
        clk = 0;
        #3.0 // 3.0 + 2.1 = 5.1 > 5.0 pulse duration
        in = 123;
        #1.1
        in = 45;
        #1 // 1 < setup time = 2
        clk = 1;
        #0
        `CHECK_TIMING(uut, violate_setup_time, 1'b1);
        #1 // 1 < hold time = 1.5
        in = 67;
        #0
        `CHECK_TIMING(uut, violate_hold_time, 1'b1);
        #2 // 1 + 2 = 3 < pulse duration = 5
        clk = 0;
        #0
        `CHECK_TIMING(uut, violate_pulse_duration, 1'b1);

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule