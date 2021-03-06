use primitives::types::*;

pub struct MisbehaviourReporter {
    /// TODO: Implement a better way to access violations than
    /// making this field public
    pub violations: Vec<ViolationType>,
}

impl MisbehaviourReporter {
    pub fn new() -> MisbehaviourReporter {
        MisbehaviourReporter { violations: vec![] }
    }

    /// Take ownership of the violation
    pub fn report(&mut self, violation: ViolationType) {
        self.violations.push(violation);
    }
}

/// TODO: Enumerate all violations and implement evidence to check that the
/// misbehaviour really took place.
#[derive(Debug)]
pub enum ViolationType {
    BadEpoch {
        message: TxFlowHash,
    },

    InvalidSignature,

    ForkAttempt {
        message_0: TxFlowHash,
        message_1: TxFlowHash,
    },
}
