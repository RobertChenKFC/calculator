`include "src/ic/ic_74ac138.v"

module demux_3to8 #(
    parameter ADDR_WIDTH = 3,
    parameter WIDTH = 1 << ADDR_WIDTH
) (
    input[ADDR_WIDTH-1:0] addr,
    output[WIDTH-1:0] sel
);
    ic_74ac138 demux(
        .p1(addr[0]),
        .p2(addr[1]),
        .p3(addr[2]),
        .p4(1'b0),
        .p5(1'b0),
        .p6(1'b1),
        .p7(sel[7]),
        .p9(sel[6]),
        .p10(sel[5]),
        .p11(sel[4]),
        .p12(sel[3]),
        .p13(sel[2]),
        .p14(sel[1]),
        .p15(sel[0])
    );
endmodule