`include "src/ic/delay.v"

// Datasheet: https://www.ti.com/lit/ds/symlink/cd74ac157.pdf
// Propagation delay for 5V, operating temperature -40C to 85C:
// A, B -> Y: max(7.7, 7.7) = 7.7 ns.
// !A/B -> Y: max(13.2, 13.2) = 13.2 ns.
// !G -> Y: max(12.3, 12.3) = 12.3 ns.
// max(7.7, 13.2, 12.3) = 13.2.
`define DELAY 13.2

(* groups = {"((('p1', 'p15', 'p2', 'p3'), ('p4',)),",
             " (('p1', 'p15', 'p5', 'p6'), ('p7',)),",
             " (('p1', 'p15', 'p11', 'p10'), ('p9',)),",
             " (('p1', 'p15', 'p14', 'p13'), ('p12',)))"} *)
`DEF_DELAY_ATTR
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
    wire[3:0] a;
    wire[3:0] b;
    wire[3:0] y;

    assign a = {p14, p11, p5, p2};
    assign b = {p13, p10, p6, p3};
    assign #`DELAY y = p15 ? 4'b0 : (p1 ? b : a);
    assign {p12, p9, p7, p4} = y;
endmodule