`include "src/ic/delay.v"

// Datasheet: https://www.ti.com/lit/ds/symlink/cd74ac138.pdf
// Propagation delay for 5V, operating temperature around 25C:
// A, B, C -> Y: max(10, 10) = 10 ns.
// G1 -> Y: max(10, 10) = 10 ns.
// !G2A, !G2B -> Y: max(9.1, 9.1) = 9.1 ns.
// max(10, 10, 9.1) = 10 ns.
`define DELAY 10

(* groups = {"((",
        "('p1', 'p2', 'p3', 'p4', 'p5', 'p6'), ",
        "('p7', 'p9', 'p10', 'p11', 'p12', 'p13', 'p14', 'p15')),)"} *)
`DEF_DELAY_ATTR
module ic_74ac138(
    input p1, // A
    input p2, // B
    input p3, // C
    input p4, // !G2A
    input p5, // !G2B
    input p6, // G1
    output p7, // Y7
    output p9, // Y6
    output p10, // Y5
    output p11, // Y4
    output p12, // Y3
    output p13, // Y2
    output p14, // Y1
    output p15 // Y0
);

    wire not_enable;
    assign not_enable = !p6 | p4 | p5;
    assign #`DELAY p15 = p3 | p2 | p1 | not_enable;
    assign #`DELAY p14 = p3 | p2 | !p1 | not_enable;
    assign #`DELAY p13 = p3 | !p2 | p1 | not_enable;
    assign #`DELAY p12 = p3 | !p2 | !p1 | not_enable;
    assign #`DELAY p11 = !p3 | p2 | p1 | not_enable;
    assign #`DELAY p10 = !p3 | p2 | !p1 | not_enable;
    assign #`DELAY p9 = !p3 | !p2 | p1 | not_enable;
    assign #`DELAY p7 = !p3 | !p2 | !p1 | not_enable;
endmodule

`undef DELAY