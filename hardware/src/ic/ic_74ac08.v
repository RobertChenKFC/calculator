`include "src/ic/delay.v"

// Datasheet: https://www.ti.com/lit/ds/symlink/sn74ac08.pdf
// Propagation delay for 5V, operating temperature around 25C:
// - t_PLH: 8.5 ns
// - t_PHL: 7.5 ns
// max(8.5, 7.5) = 8.5.
`define DELAY 8.5

(* groups = {"((('p1', 'p2'), ('p3',)),",
             " (('p4', 'p5'), ('p6',)),",
             " (('p9', 'p10'), ('p8',)),",
             " (('p12', 'p13'), ('p11',)))"} *)
`DEF_DELAY_ATTR
module ic_74ac08(
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

    assign a = {p12, p9, p4, p1};
    assign b = {p13, p10, p5, p2};
    assign #`DELAY y = a & b;
    assign {p11, p8, p6, p3} = y;
endmodule

`undef DELAY