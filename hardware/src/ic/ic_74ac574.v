module ic_74ac574(
    input p1, // !OE
    input p2, // 1D
    input p3, // 2D
    input p4, // 3D
    input p5, // 4D
    input p6, // 5D
    input p7, // 6D
    input p8, // 7D
    input p9, // 8D
    input p11, // CLK
    output p12, // 8Q
    output p13, // 7Q
    output p14, // 6Q
    output p15, // 5Q
    output p16, // 4Q
    output p17, // 3Q
    output p18, // 2Q
    output p19 // 1Q
);
`ifdef TEST_TIMING
    reg violate_setup_time;
    reg violate_pulse_duration;
    reg violate_hold_time;

    initial begin
        violate_setup_time = 1'b0;
        violate_pulse_duration = 1'b0;
        violate_hold_time = 1'b0;
    end
`endif


    wire[7:0] in;
    wire[7:0] out;
    wire clk;
    wire not_oe;
    reg[7:0] data;

    // Datasheet: https://www.ti.com/lit/ds/symlink/sn74ac574.pdf
    // 5V, 25C:
    // - Pulse duration: 5 ns
    // - Setup time: 2 ns
    // - Hold time: 1.5 ns
    // - Propagation delay: max(11, 9.5, 9, 9, 10, 8.5) = 11 ms.

    assign in = {p2, p3, p4, p5, p6, p7, p8, p9};
    assign {p19, p18, p17, p16, p15, p14, p13, p12} = out;
    assign clk = p11;
    assign not_oe = p1;

    realtime clk_start;
    realtime in_start;

    always @(posedge clk) begin
        data <= in;
        clk_start = $realtime;
        if (clk_start - in_start < 2) begin
`ifdef TEST_TIMING
            violate_setup_time = 1'b1;
`else
            $display("Setup time violation for 74AC574: expected >= 2 ns, ",
                     "got %f ns", clk_start - in_start);
            $fatal(1);
`endif
        end
    end

    always @(negedge clk) begin
        if (clk_start > 0 && $realtime - clk_start < 5) begin
`ifdef TEST_TIMING
            violate_pulse_duration = 1'b1;
`else
            $display("Pulse duration violation for 74AC574: expected >= 5 ns, ",
                     "got %f ns", $realtime - clk_start);
            $fatal(1);
`endif
        end
    end

    always @(in) begin
        if (clk_start > 0 && $realtime - clk_start < 1.5) begin
`ifdef TEST_TIMING
            violate_hold_time = 1'b1;
`else
            $display("Hold time violation for 74AC574: expected >= 1.5 ns, ",
                     "got %f ns", $realtime - clk_start);
            $fatal(1);
`endif
        end
        in_start = $realtime;
    end

    assign #11 out = not_oe ? 8'bz : data;
endmodule