`include "src/ic/ic_74ac283.v"

module adder #(
    parameter WIDTH=8
) (
    input[WIDTH-1:0] x,
    input[WIDTH-1:0] y,
    output[WIDTH-1:0] z
);
    wire c_out_low;
    ic_74ac283 adder_low(
        .p1(z[1]), // s1
        .p2(y[1]), // b1
        .p3(x[1]), // a1
        .p4(z[0]), // s0
        .p5(x[0]), // a0
        .p6(y[0]), // b0
        .p7(1'b0), // c_in
        .p8(1'b0), // gnd
        .p9(c_out_low), // c_out
        .p10(z[3]), // s3
        .p11(y[3]), // b3
        .p12(x[3]), // a3
        .p13(z[2]), // s2
        .p14(x[2]), // a2
        .p15(y[2]), // b2
        .p16(1'b1) // vcc
    );
    ic_74ac283 adder_high(
        .p1(z[5]), // s1
        .p2(y[5]), // b1
        .p3(x[5]), // a1
        .p4(z[4]), // s0
        .p5(x[4]), // a0
        .p6(y[4]), // b0
        .p7(c_out_low), // c_in
        .p8(1'b0), // gnd
        .p9(), // c_out
        .p10(z[7]), // s3
        .p11(y[7]), // b3
        .p12(x[7]), // a3
        .p13(z[6]), // s2
        .p14(x[6]), // a2
        .p15(y[6]), // b2
        .p16(1'b1) // vcc
    );
endmodule