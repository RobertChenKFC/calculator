`include "src/ic/ic_74ac574.v"

module register #(
    parameter WIDTH=8
) (
    input clk,
    input not_oe,
    input[WIDTH-1:0] in,
    output[WIDTH-1:0] out
);
    ic_74ac574 data(
        .p1(not_oe),
        .p2(in[0]),
        .p3(in[1]),
        .p4(in[2]),
        .p5(in[3]),
        .p6(in[4]),
        .p7(in[5]),
        .p8(in[6]),
        .p9(in[7]),
        .p11(clk),
        .p12(out[7]),
        .p13(out[6]),
        .p14(out[5]),
        .p15(out[4]),
        .p16(out[3]),
        .p17(out[2]),
        .p18(out[1]),
        .p19(out[0])
    );
endmodule