module ic_74ac283(
    output p1, // s1
    input p2, // b1
    input p3, // a1
    output p4, // s0
    input p5, // a0
    input p6, // b0
    input p7, // c_in
    output p9, // c_out
    output p10, // s3
    input p11, // b3
    input p12, // a3
    output p13, // s2
    input p14, // a2
    input p15 // b2
);
    wire[3:0] a;
    wire[3:0] b;
    wire[4:0] s;
    assign a = {p12, p14, p3, p5};
    assign b = {p11, p15, p2, p6};
    // Datasheet: https://www.ti.com/lit/ds/symlink/cd74act283.pdf?ts=1779756255166
    // Propagation delay for 5V, operating temperature -40C to 85C:
    // - a_n/b_n to c_out, c_in to s_n, c_in to c_out: 16 ns
    // - a_b/b_n to s_n: 16.5 ns
    // max(16, 16.5) = 16.5.
    assign #16.5 s = a + b + p7;
    assign {p9, p10, p13, p1, p4} = s;
endmodule