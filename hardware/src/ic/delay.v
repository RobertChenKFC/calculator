`ifndef DELAY_V
`define DELAY_V

`ifdef SYNTHESIS
`define DEF_DELAY_ATTR (* delay = `"`DELAY`" *)
`else
`define DEF_DELAY_ATTR
`endif

`endif // DELAY_V