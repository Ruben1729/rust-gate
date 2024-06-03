use crate::error::LatchError;
use crate::logic_gate::*;
use crate::latch::*;
use crate::SignalEdge;

trait FlipFlop {
    fn set(&mut self, a: bool, b: bool, clk: bool) -> Result<(), LatchError>;
    fn set_n(&mut self, na: bool, nb: bool, clk: bool) -> Result<(), LatchError>;
    fn evaluate(&mut self);
    fn get_q(&self) -> bool;
    fn get_nq(&self) -> bool;
}

#[derive(Debug)]
pub struct SrFlipFlop {
    clk: bool,
    sr_latch: SrLatch,
    edge_trigger: SignalEdge
}

impl FlipFlop for SrFlipFlop {
    fn set(&mut self, s: bool, r: bool, clk: bool) -> Result<(), LatchError> {
        let did_change = XorGate::evaluate(clk, self.clk);

        match self.edge_trigger {
            SignalEdge::Rising if did_change && !clk => self.sr_latch.set(s, r),
            SignalEdge::Falling if did_change && clk => self.sr_latch.set(s, r),
            SignalEdge::Change => self.sr_latch.set(s, r),
            _ => Ok(())
        }
    }

    fn set_n(&mut self, na: bool, nb: bool, clk: bool) -> Result<(), LatchError> {
        self.set(NotGate::evaluate(na), NotGate::evaluate(nb), clk)
    }

    fn evaluate(&mut self) {
        self.sr_latch.evaluate()
    }

    fn get_q(&self) -> bool {
        self.sr_latch.get_q()
    }

    fn get_nq(&self) -> bool {
        self.sr_latch.get_nq()
    }
}
