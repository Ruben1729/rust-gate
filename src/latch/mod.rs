pub mod error;
use crate::logic_gate::*;
use self::error::*;

pub trait Latch {
	fn set(&mut self, a: bool, b: bool) -> Result<(), LatchError>;
	fn set_n(&mut self, na: bool, nb: bool) -> Result<(), LatchError>;
	fn evaluate(&mut self);
	fn get_q(&self) -> bool;
	fn get_nq(&self) -> bool;
}

#[derive(Debug)]
pub struct SrLatch {
	ns: bool,
	nr: bool,
	q: bool,
	nq: bool
}

impl Latch for SrLatch {
	fn set(&mut self, s: bool, r: bool) -> Result<(), LatchError> {
		self.set_n(NotGate::evaluate(s), NotGate::evaluate(r))
	}

	fn set_n(&mut self, ns: bool, nr: bool) -> Result<(), LatchError> {
		if ns == false && nr == false {
			return Err(LatchError::InvalidInput)
		}

		self.ns = ns;
		self.nr = nr;
		self.evaluate();

		Ok(())
	}

	fn evaluate(&mut self) {
		let next_nq = NandGate::evaluate(self.nr, self.q);
		let next_q = NandGate::evaluate(self.ns, next_nq);

		self.q = next_q;
		self.nq = NotGate::evaluate(self.q);
	}
	
	fn get_q(&self) -> bool {
		self.q
	}
	
	fn get_nq(&self) -> bool {
		self.nq
	}
}

impl Default for SrLatch {
	fn default() -> Self {
		Self { ns: true, nr: true, q: true, nq: false }
	}
}

#[derive(Debug)]
pub struct DLatch {
	d: bool,
	e: bool,
	sr_latch: SrLatch,
}

impl Latch for DLatch {
	fn set(&mut self, d: bool, e: bool) -> Result<(), LatchError> {
		self.d = d;
		self.e = e;

		self.evaluate();
		Ok(())
	}
	
	fn set_n(&mut self, nd: bool, ne: bool) -> Result<(), LatchError> {
		self.set(NotGate::evaluate(nd), NotGate::evaluate(ne))
	}

	fn evaluate(&mut self) {
		let ns = NandGate::evaluate(self.d, self.e);
		let nr = NandGate::evaluate(NotGate::evaluate(self.d), self.e);

		// We should never have ns = nr = 0
		self.sr_latch.set_n(ns, nr).expect("Unable to update SR Latch inside D Latch.");
	}
	
	fn get_q(&self) -> bool {
		self.sr_latch.get_q()
	}
	
	fn get_nq(&self) -> bool {
		self.sr_latch.get_nq()
	}
}

impl Default for DLatch {
	fn default() -> Self {
		Self { d: true, e: true, sr_latch: SrLatch::default() }
	}
}

#[derive(Debug)]
pub struct JkLatch {
    j: bool,
    k: bool,
    sr_latch: SrLatch,
}

impl Latch for JkLatch {
	fn set(&mut self, j: bool, k: bool) -> Result<(), LatchError> {
		self.j = j;
		self.k = k;
		self.evaluate();

		Ok(())
	}
	
	fn set_n(&mut self, nj: bool, nk: bool) -> Result<(), LatchError> {
		self.set(NotGate::evaluate(nj), NotGate::evaluate(nk))
	}

	fn evaluate(&mut self) {
		let ns = NandGate::evaluate(self.j, self.sr_latch.get_nq());
		let nr = NandGate::evaluate(self.k, self.sr_latch.get_q());
		self.sr_latch.set_n(ns, nr).expect("Unable to update SR Latch inside JK Latch.");
	}

    fn get_q(&self) -> bool {
        self.sr_latch.get_q()
	}

	fn get_nq(&self) -> bool {
		self.sr_latch.get_nq()
	}
}

impl Default for JkLatch {
	fn default() -> Self {
		Self {
			j: true,
			k: false,
			sr_latch: SrLatch::default()
		}
	}
}

#[derive(Debug)]
pub struct TLatch {
	t: bool,
	jk_latch: JkLatch
}

impl TLatch {
	fn set(&mut self, t: bool) -> Result<(), LatchError> {
		self.t = t;
		self.jk_latch.set(t, t)
	}

	fn set_n(&mut self, nt: bool, _: bool) -> Result<(), LatchError> {
		self.set(NotGate::evaluate(nt))
	}

	fn evaluate(&mut self) {
		self.jk_latch.evaluate()
	}

	fn get_q(&self) -> bool {
		self.jk_latch.get_q()
	}

	fn get_nq(&self) -> bool {
		self.jk_latch.get_nq()
	}
}

impl Default for TLatch {
	fn default() -> Self {
		Self { t: false, jk_latch: JkLatch::default() }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sr_latch() {
		let mut sr_latch = SrLatch::default();

		// Initial state
		assert_eq!(sr_latch.get_q(), true);
		assert_eq!(sr_latch.get_nq(), false);

		// Set
		sr_latch.set(true, false).expect("Unable to set s, r to 1, 0");
		assert_eq!(sr_latch.get_q(), true);
		assert_eq!(sr_latch.get_nq(), false);

		// Reset
		sr_latch.set(false, true).expect("Unable to set s, r to 0, 1");
		assert_eq!(sr_latch.get_q(), false);
		assert_eq!(sr_latch.get_nq(), true);

		// No change
		sr_latch.set(false, false).expect("Unable to set s, r to 0, 0");
		assert_eq!(sr_latch.get_q(), false);
		assert_eq!(sr_latch.get_nq(), true);
		
		sr_latch.set(true, false).expect("Unable to set s, r to 1, 0");
		sr_latch.set(false, false).expect("Unable to set s, r to 0, 0");
		assert_eq!(sr_latch.get_q(), true);
		assert_eq!(sr_latch.get_nq(), false);

		// Invalid state
		let result = sr_latch.set(true, true);
		assert!(result.is_err());
		assert_eq!(result.err(), Some(LatchError::InvalidInput));
	}

	#[test]
	fn test_d_latch() {
		let mut d_latch = DLatch::default();
		
		// Initial state
		assert_eq!(d_latch.get_q(), true);
		assert_eq!(d_latch.get_nq(), false);
		
		// Reset Q to 0
		d_latch.set(false, true).expect("Unable to set e, d to 0, 1");
		assert_eq!(d_latch.get_q(), false);
		assert_eq!(d_latch.get_nq(), true);
		
		// Set Q to 1
		d_latch.set(true, true).expect("Unable to set e, d to 1, 1");
		assert_eq!(d_latch.get_q(), true);
		assert_eq!(d_latch.get_nq(), false);
		
		// No change
		d_latch.set(false, false).expect("Unable to set e, d to 0, 0");
		assert_eq!(d_latch.get_q(), true);
		assert_eq!(d_latch.get_nq(), false);
		
		d_latch.set(false, true).expect("Unable to set e, d to 0, 1");
		d_latch.set(true, false).expect("Unable to set e, d to 1, 0");
		assert_eq!(d_latch.get_q(), false);
		assert_eq!(d_latch.get_nq(), true);
	}

	#[test]
	fn test_jk_latch() {
		let mut jk_latch = JkLatch::default();

		// Initial state
		assert_eq!(jk_latch.get_q(), true);
		assert_eq!(jk_latch.get_nq(), false);

		// Reset
		jk_latch.set(true, false).unwrap();
		assert_eq!(jk_latch.get_q(), true);
		assert_eq!(jk_latch.get_nq(), false);

		// Set
		jk_latch.set(false, true).unwrap();
		assert_eq!(jk_latch.get_q(), false);
		assert_eq!(jk_latch.get_nq(), true);

		// Save
		jk_latch.set(false, false).unwrap();
		assert_eq!(jk_latch.get_q(), false);
		assert_eq!(jk_latch.get_nq(), true);

		jk_latch.set(true, false).unwrap();
		jk_latch.set(false, false).unwrap();
		assert_eq!(jk_latch.get_q(), true);
		assert_eq!(jk_latch.get_nq(), false);

		// Toggle
		jk_latch.set(true, true).unwrap();
		assert_eq!(jk_latch.get_q(), false);
		assert_eq!(jk_latch.get_nq(), true);

		jk_latch.set(true, true).unwrap();
		assert_eq!(jk_latch.get_q(), true);
		assert_eq!(jk_latch.get_nq(), false);

		jk_latch.set(true, true).unwrap();
		assert_eq!(jk_latch.get_q(), false);
		assert_eq!(jk_latch.get_nq(), true);
	}

	#[test]
	fn test_t_latch() {
		let mut t_latch = TLatch::default();

		// Initial state
		assert_eq!(t_latch.get_q(), true);
		assert_eq!(t_latch.get_nq(), false);

		// Toggle
		t_latch.set(true).unwrap();
		assert_eq!(t_latch.get_q(), false);
		assert_eq!(t_latch.get_nq(), true);

		t_latch.set(true).unwrap();
		assert_eq!(t_latch.get_q(), true);
		assert_eq!(t_latch.get_nq(), false);

		// Save
		t_latch.set(false).unwrap();
		assert_eq!(t_latch.get_q(), true);
		assert_eq!(t_latch.get_nq(), false);

		t_latch.set(true).unwrap();
		t_latch.set(false).unwrap();
		assert_eq!(t_latch.get_q(), false);
		assert_eq!(t_latch.get_nq(), true);
	}
}
