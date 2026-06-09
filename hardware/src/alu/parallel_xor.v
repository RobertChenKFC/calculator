`include "src/ic/ic_74ac86.v"

module parallel_xor #(
    parameter WIDTH=8
) (
    input[WIDTH-1:0] x,
    input[WIDTH-1:0] y,
    output[WIDTH-1:0] z
);
    ic_74ac86 parallel_xor_low(
        .p1(x[0]),
        .p2(y[0]),
        .p3(z[0]),
        .p4(x[1]),
        .p5(y[1]),
        .p6(z[1]),
        .p8(z[2]),
        .p9(x[2]),
        .p10(y[2]),
        .p11(z[3]),
        .p12(x[3]),
        .p13(y[3])
    );
    ic_74ac86 parallel_xor_high(
        .p1(x[4]),
        .p2(y[4]),
        .p3(z[4]),
        .p4(x[5]),
        .p5(y[5]),
        .p6(z[5]),
        .p8(z[6]),
        .p9(x[6]),
        .p10(y[6]),
        .p11(z[7]),
        .p12(x[7]),
        .p13(y[7])
    );
endmodule