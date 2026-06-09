module ic_74ac86(
    input p1, // 1A
    input p2, // 1B
    output p3, // 1Y
    input p4, // 2A
    input p5, // 2B
    output p6, // 2Y
    output p8, // 3Y
    input p9, // 3A
    input p10, // 3B
    output p11, // 4Y
    input p12, // 4A
    input p13 // 4B
);
    // Datasheet: https://www.ti.com/lit/ds/symlink/sn74ac86.pdf
    // Propagation delay for 5V, operating temperature around 25C:
    // - t_PLH: 9 ns
    // - t_PHL: 9.5 ns
    // max(9, 9.5) = 9.5.
    assign #9.5 p3 = p1 ^ p2;
    assign #9.5 p6 = p4 ^ p5;
    assign #9.5 p8 = p9 ^ p10;
    assign #9.5 p11 = p12 ^ p13;
endmodule