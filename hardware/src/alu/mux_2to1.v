`include "src/ic/ic_74ac157.v"

module mux_2to1 #(
    parameter WIDTH=8
) (
    input sel,
    input[WIDTH-1:0] x,
    input[WIDTH-1:0] y,
    output[WIDTH-1:0] z
);
    ic_74ac157 mux_2to1_low(
        .p1(sel),
        .p2(x[0]),
        .p3(y[0]),
        .p4(z[0]),
        .p5(x[1]),
        .p6(y[1]),
        .p7(z[1]),
        .p9(z[2]),
        .p10(y[2]),
        .p11(x[2]),
        .p12(z[3]),
        .p13(y[3]),
        .p14(x[3]),
        .p15(1'b0)
    );
    ic_74ac157 mux_2to1_high(
        .p1(sel),
        .p2(x[4]),
        .p3(y[4]),
        .p4(z[4]),
        .p5(x[5]),
        .p6(y[5]),
        .p7(z[5]),
        .p9(z[6]),
        .p10(y[6]),
        .p11(x[6]),
        .p12(z[7]),
        .p13(y[7]),
        .p14(x[7]),
        .p15(1'b0)
    );
endmodule