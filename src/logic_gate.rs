pub trait Gate {
    fn evaluate(a: bool, b: bool) -> bool;
}

pub struct AndGate;

impl Gate for AndGate {
    fn evaluate(a: bool, b: bool) -> bool {
        a && b
    }
}

pub struct OrGate;

impl Gate for OrGate {
    fn evaluate(a: bool, b: bool) -> bool {
        a || b
    }
}

pub struct NotGate;

impl NotGate {
    fn evaluate(a: bool) -> bool {
        !a
    }
}

pub struct NandGate;

impl Gate for NandGate {
    fn evaluate(a: bool, b: bool) -> bool {
        !(a && b)
    }
}

pub struct NorGate;

impl Gate for NorGate {
    fn evaluate(a: bool, b: bool) -> bool {
        !(a || b)
    }
}

pub struct XorGate;

impl Gate for XorGate {
    fn evaluate(a: bool, b: bool) -> bool {
        a ^ b
    }
}

pub struct XnorGate;

impl Gate for XnorGate {
    fn evaluate(a: bool, b: bool) -> bool {
        !(a ^ b)
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_and_gate() {
		assert_eq!(AndGate::evaluate(true, true), true);
		assert_eq!(AndGate::evaluate(true, false), false);
		assert_eq!(AndGate::evaluate(false, true), false);
		assert_eq!(AndGate::evaluate(false, false), false);
	}

	#[test]
	fn test_or_gate() {
		assert_eq!(OrGate::evaluate(true, true), true);
		assert_eq!(OrGate::evaluate(true, false), true);
		assert_eq!(OrGate::evaluate(false, true), true);
		assert_eq!(OrGate::evaluate(false, false), false);
	}

	#[test]
	fn test_not_gate() {
		assert_eq!(NotGate::evaluate(true), false);
		assert_eq!(NotGate::evaluate(false), true);
	}

	#[test]
	fn test_nand_gate() {
		assert_eq!(NandGate::evaluate(true, true), false);
		assert_eq!(NandGate::evaluate(true, false), true);
		assert_eq!(NandGate::evaluate(false, true), true);
		assert_eq!(NandGate::evaluate(false, false), true);
	}

	#[test]
	fn test_nor_gate() {
		assert_eq!(NorGate::evaluate(true, true), false);
		assert_eq!(NorGate::evaluate(true, false), false);
		assert_eq!(NorGate::evaluate(false, true), false);
		assert_eq!(NorGate::evaluate(false, false), true);
	}

	#[test]
	fn test_xor_gate() {
		assert_eq!(XorGate::evaluate(true, true), false);
		assert_eq!(XorGate::evaluate(true, false), true);
		assert_eq!(XorGate::evaluate(false, true), true);
		assert_eq!(XorGate::evaluate(false, false), false);
	}

	#[test]
	fn test_xnor_gate() {
		assert_eq!(XnorGate::evaluate(true, true), true);
		assert_eq!(XnorGate::evaluate(true, false), false);
		assert_eq!(XnorGate::evaluate(false, true), false);
		assert_eq!(XnorGate::evaluate(false, false), true);
	}
}
