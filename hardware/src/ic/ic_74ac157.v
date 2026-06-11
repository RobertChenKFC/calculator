module ic_74ac157(
    input p1, // !A/B
    input p2, // 1A
    input p3, // 1B
    output p4, // 1Y
    input p5, // 2A
    input p6, // 2B
    output p7, // 2Y
    output p9, // 3Y
    input p10, // 3B
    input p11, // 3A
    output p12, // 4Y
    input p13, // 4B
    input p14, // 4A
    input p15 // !G
);
    // Datasheet: https://www.ti.com/lit/ds/symlink/cd74ac157.pdf
    // Propagation delay for 5V, operating temperature -40C to 85C:
    // - A or B to Y, t_PLH: 7.7 ns
    // - A or B to Y, t_PHL: 7.7 ns
    // - !A/B to Y, t_PLH: 13.2 ns
    // - !A/B to Y, t_PHL: 13.2 ns
    // - !G to Y, t_PLH: 12.3 ns
    // - !G to Y, t_PHL: 12.3 ns
    // Note that we do not include the propagation delay of !G in the
    // calculation, because we are not expected to use that input (we will
    // permanently tie that to low).
    // max(7.7, 7.7, 13.2, 13.2) = 13.2.
    wire[3:0] a;
    wire[3:0] b;
    wire[3:0] y;

    assign a = {p2, p5, p11, p14};
    assign b = {p3, p6, p10, p13};
    assign #13.2 y = p15 ? 4'b0 : (p1 ? b : a);
    assign {p4, p7, p9, p12} = y;
endmodule