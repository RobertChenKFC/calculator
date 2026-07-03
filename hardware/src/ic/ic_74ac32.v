`include "src/ic/delay.v"

// Datasheet: https://www.ti.com/lit/ds/scas528h/scas528h.pdf
// Propagation delay for 5V, operating temperature around 25C:
// - t_PLH: 9 ns
// - t_PHL: 8.5 ns
// max(9, 8.5) = 9.
`define DELAY 9

(* groups = {"((('p1', 'p2'), ('p3',)),",
             " (('p4', 'p5'), ('p6',)),",
             " (('p9', 'p10'), ('p8',)),",
             " (('p12', 'p13'), ('p11',)))"} *)
`DEF_DELAY_ATTR
module ic_74ac32(
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
    wire[3:0] a;
    wire[3:0] b;
    wire[3:0] y;

    assign a = {p1, p4, p9, p12};
    assign b = {p2, p5, p10, p13};
    assign #`DELAY y = a | b;
    assign {p3, p6, p8, p11} = y;
endmodule