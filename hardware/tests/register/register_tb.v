`timescale 1ns/1ps
`define TEST_TIMING
`include "src/register/register.v"

module register_tb;
    localparam WIDTH = 8;

    reg not_oe;
    reg clk;
    reg[WIDTH-1:0] in;
    wire[WIDTH-1:0] actual;
    reg[WIDTH-1:0] expected;

    register uut(
        .clk(clk),
        .not_oe(not_oe),
        .in(in),
        .out(actual)
    );

    integer i;
    integer j;
    initial begin
        not_oe = 1'b0;
        for (i = 0; i < (1 << WIDTH); i = i + 1) begin
            clk = 0;
            in = i;
            #2 // setup time
            // This checks that the register holds the value, it does not
            // change even though the input has changes. We ignore the first
            // iteration since there's no default value for the register.
            expected = i - 1;
            if (i > 0 && expected !== actual) begin
                $display("FAIL: hold: expected %d, got %d", expected, actual);
                $fatal(1);
            end
            #5 // pulse width - setup time
            clk = 1;
            #11 // max(hold time, propagation delay)
            #0
            // This checks that after sufficient time, the register value
            // changes according to the input.
            // TODO: this check fails. Continue here
            expected = i;
            if (expected !== actual) begin
                $display("FAIL: propagation: expected %d, got %d", expected,
                        actual);
                $fatal(1);
            end
        end

        not_oe = 1'b1;
        #11
        #0
        if (actual !== {WIDTH{1'bz}}) begin
            $display("FAIL: output disabled: expected z, got %d", actual);
        end

        $display("SUCCESS: passed all tests");
        $finish;
    end
endmodule