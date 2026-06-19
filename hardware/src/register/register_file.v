`include "src/alu/parallel_or.v"
`include "src/common/demux_3to8.v"
`include "src/register/register.v"

module register_file #(
    parameter WIDTH=8,
    parameter IDX_WIDTH=2
) (
    input clk,
    input not_we,
    input[IDX_WIDTH-1:0] rd_idx,
    input[IDX_WIDTH-1:0] rs0_idx,
    input[IDX_WIDTH-1:0] rs1_idx,
    input[WIDTH-1:0] in,
    output[WIDTH-1:0] out0,
    output[WIDTH-1:0] out1
);
    localparam NUM_REGS = 1 << IDX_WIDTH;

    // `rd_idx` represents the index of the destination register to write to.
    // `not_sel_we` decodes `rd_idx` as a one-cold vector, so every bit of
    // this vector is 1 except the `rd_idx`-th bit.
    wire[7:0] not_sel_we;
    demux_3to8 demux_rd(
        .addr({1'b0, rd_idx}),
        .sel(not_sel_we)
    );
    // For each register i, we want to compute:
    //   clk[i] = clk | not_we | not_sel_we[i]
    // This is because the register is written to on the rising edge of the
    // clock. Therefore, if either `not_we` is high (so write is disabled for
    // the whole register file) or `not_sel_we[i]` is high (so register i is not
    // selected), `clk[i]` will remain high, so there will be no rising edge.
    // Note that we use one parallel or gate to compute this, by chaining the
    // output of `clk | we` to the input of the other bits of the or gate.
    wire[NUM_REGS-1:0] we_gated_clk;
    wire[NUM_REGS-1:0] reg_clks;
    parallel_or gate_sel_we(
        .x({{NUM_REGS{clk}}, we_gated_clk}),
        .y({{NUM_REGS{not_we}}, not_sel_we[NUM_REGS-1:0]}),
        .z({we_gated_clk, reg_clks})
    );

    genvar pair_idx;
    genvar reg_idx;
    wire[IDX_WIDTH-1:0] rs_indices[1:0];
    assign rs_indices[0] = rs0_idx;
    assign rs_indices[1] = rs1_idx;
    // We have to use a flattened vector instead of an array of vectors because
    // of the "Yosys array bug", see this report for more detail:
    // https://docs.google.com/document/d/1rTGmMuzfI23QpsAkw8GNdFL6gcjZ3_RabjpzuoBNSwI/edit?tab=t.hzy92lfg5c3r 
    wire[2*WIDTH-1:0] outs;
    assign out0 = outs[0+:WIDTH];
    assign out1 = outs[WIDTH+:WIDTH];
    for (pair_idx = 0; pair_idx < 2; pair_idx = pair_idx + 1) begin
        // Here, we use a pair of duplicate registers that gets written the same
        // value for each register index. Therefore, they are connected to the
        // same write enable for each pair. However, they are connected to
        // different output enables and different outputs so that this register
        // file has 2 read ports.
        wire[7:0] not_oes;
        // Similar to `not_sel_we`, `not_oes` is a one-cold vector where all but
        // bit i is 0 if register i is selected.
        demux_3to8 sel_oe(
            .addr({1'b0, rs_indices[pair_idx]}),
            .sel(not_oes)
        );
        for (reg_idx = 0; reg_idx < NUM_REGS; reg_idx = reg_idx + 1) begin
            register reg_i(
                .clk(reg_clks[reg_idx]),
                .not_oe(not_oes[reg_idx]),
                .in(in),
                .out(outs[pair_idx*WIDTH+:WIDTH])
            );
        end
    end
endmodule