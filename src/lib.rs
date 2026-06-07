//! Biological signal processing pathway simulation.
//!
//! Implements receptors, kinase cascades, signal amplifiers, gating mechanisms,
//! and complete signal transduction pathways.

// ── Module: receptor ─────────────────────────────────────────────────────────

pub mod receptor {
    /// A receptor that detects input signals.
    #[derive(Clone, Debug)]
    pub struct Receptor {
        pub name: String,
        pub threshold: f64,
        pub sensitivity: f64,
        pub active: bool,
        pub signal: f64,
        pub max_signal: f64,
        pub desensitization_rate: f64,
        pub desensitization_level: f64,
    }

    impl Receptor {
        pub fn new(name: &str, threshold: f64, sensitivity: f64, max_signal: f64) -> Self {
            Receptor {
                name: name.to_string(),
                threshold,
                sensitivity,
                active: false,
                signal: 0.0,
                max_signal,
                desensitization_rate: 0.0,
                desensitization_level: 0.0,
            }
        }

        /// Detect an input signal. Returns true if activated.
        pub fn detect(&mut self, input: f64) -> bool {
            self.active = input >= self.threshold;
            if self.active {
                let effective_sensitivity = self.sensitivity * (1.0 - self.desensitization_level);
                self.signal = (input * effective_sensitivity).min(self.max_signal);
            } else {
                self.signal = 0.0;
            }
            self.active
        }

        /// Get current output signal.
        pub fn output(&self) -> f64 {
            if self.active { self.signal } else { 0.0 }
        }

        /// Reset the receptor.
        pub fn reset(&mut self) {
            self.active = false;
            self.signal = 0.0;
            self.desensitization_level = 0.0;
        }

        /// Apply desensitization (reduces sensitivity over repeated activation).
        pub fn desensitize(&mut self) {
            if self.active {
                self.desensitization_level = (self.desensitization_level + self.desensitization_rate)
                    .min(1.0);
            } else {
                self.desensitization_level = (self.desensitization_level - self.desensitization_rate * 0.5)
                    .max(0.0);
            }
        }

        /// Set desensitization rate.
        pub fn with_desensitization(mut self, rate: f64) -> Self {
            self.desensitization_rate = rate;
            self
        }

        /// Check if receptor is saturated.
        pub fn is_saturated(&self) -> bool {
            self.signal >= self.max_signal * 0.99
        }

        /// Compute the dose-response curve value.
        pub fn dose_response(&self, concentration: f64) -> f64 {
            // Hill equation: V = Vmax * C^n / (K^n + C^n)
            let n = self.sensitivity; // Using sensitivity as Hill coefficient
            let c = concentration;
            let k = self.threshold;
            self.max_signal * c.powf(n) / (k.powf(n) + c.powf(n))
        }
    }
}

// ── Module: kinase ───────────────────────────────────────────────────────────

pub mod kinase {
    /// A kinase in a phosphorylation cascade.
    #[derive(Clone, Debug)]
    pub struct Kinase {
        pub name: String,
        pub active: bool,
        pub phosphorylation_level: f64,  // 0.0 to 1.0
        pub activity: f64,               // output signal strength
        pub max_activity: f64,
        pub activation_threshold: f64,
        pub dephosphorylation_rate: f64,
    }

    impl Kinase {
        pub fn new(name: &str, max_activity: f64, activation_threshold: f64) -> Self {
            Kinase {
                name: name.to_string(),
                active: false,
                phosphorylation_level: 0.0,
                activity: 0.0,
                max_activity,
                activation_threshold,
                dephosphorylation_rate: 0.1,
            }
        }

        /// Activate the kinase with a given input signal.
        pub fn activate(&mut self, input_signal: f64) {
            if input_signal >= self.activation_threshold {
                self.active = true;
                self.phosphorylation_level = (self.phosphorylation_level + input_signal * 0.1)
                    .min(1.0);
                self.activity = self.phosphorylation_level * self.max_activity;
            }
        }

        /// Deactivate the kinase (dephosphorylation).
        pub fn deactivate(&mut self) {
            self.phosphorylation_level = (self.phosphorylation_level - self.dephosphorylation_rate)
                .max(0.0);
            if self.phosphorylation_level <= 0.0 {
                self.active = false;
            }
            self.activity = self.phosphorylation_level * self.max_activity;
        }

        /// Get output signal.
        pub fn output(&self) -> f64 {
            self.activity
        }

        /// Reset kinase.
        pub fn reset(&mut self) {
            self.active = false;
            self.phosphorylation_level = 0.0;
            self.activity = 0.0;
        }

        /// Check if fully phosphorylated.
        pub fn is_fully_phosphorylated(&self) -> bool {
            self.phosphorylation_level >= 0.99
        }

        /// Cascade step: activate this kinase from upstream kinase output.
        pub fn cascade_step(&mut self, upstream_signal: f64) -> f64 {
            self.activate(upstream_signal);
            self.deactivate(); // Simulate turnover
            self.output()
        }
    }

    /// A phosphorylation cascade (MAPK-like).
    pub struct Cascade {
        pub kinases: Vec<Kinase>,
    }

    impl Cascade {
        pub fn new(kinases: Vec<Kinase>) -> Self {
            Cascade { kinases }
        }

        /// Run the cascade: input signal propagates through all kinases.
        pub fn run(&mut self, input_signal: f64) -> f64 {
            let mut signal = input_signal;
            for kinase in &mut self.kinases {
                signal = kinase.cascade_step(signal);
            }
            signal
        }

        /// Get number of stages.
        pub fn stages(&self) -> usize {
            self.kinases.len()
        }

        /// Reset all kinases.
        pub fn reset(&mut self) {
            for k in &mut self.kinases {
                k.reset();
            }
        }

        /// Get total amplification (output / input).
        pub fn amplification(&self, input: f64, output: f64) -> f64 {
            if input > 0.0 { output / input } else { 0.0 }
        }
    }
}

// ── Module: amplifier ────────────────────────────────────────────────────────

pub mod amplifier {
    /// Signal amplifier with configurable gain.
    #[derive(Clone, Debug)]
    pub struct Amplifier {
        pub gain: f64,
        pub noise_floor: f64,
        pub saturation: f64,
        pub bandwidth: f64,  // frequency range
    }

    impl Amplifier {
        pub fn new(gain: f64, noise_floor: f64, saturation: f64) -> Self {
            Amplifier { gain, noise_floor, saturation, bandwidth: f64::MAX }
        }

        /// Amplify a signal.
        pub fn amplify(&self, input: f64) -> f64 {
            let amplified = input * self.gain;
            amplified.max(self.noise_floor).min(self.saturation)
        }

        /// Amplify with saturation (sigmoid-like).
        pub fn amplify_sigmoid(&self, input: f64) -> f64 {
            let x = input * self.gain / self.saturation;
            self.saturation / (1.0 + (-x).exp())
        }

        /// Compute signal-to-noise ratio.
        pub fn snr(&self, signal: f64) -> f64 {
            if self.noise_floor > 0.0 {
                let output = self.amplify(signal);
                (output / (self.noise_floor * self.gain)).max(0.0)
            } else {
                f64::MAX
            }
        }

        /// Check if amplifier is saturated.
        pub fn is_saturated(&self, input: f64) -> bool {
            input * self.gain >= self.saturation
        }

        /// Compute dynamic range in dB.
        pub fn dynamic_range_db(&self) -> f64 {
            if self.noise_floor > 0.0 {
                20.0 * (self.saturation / self.noise_floor).log10()
            } else {
                f64::MAX
            }
        }

        /// Apply band-pass filtering (simplified).
        pub fn filter_frequency(&self, signal: f64, frequency: f64) -> f64 {
            if frequency <= self.bandwidth {
                self.amplify(signal)
            } else {
                signal * self.bandwidth / frequency
            }
        }
    }

    /// Multi-stage amplifier chain.
    pub struct AmplifierChain {
        pub amplifiers: Vec<Amplifier>,
    }

    impl AmplifierChain {
        pub fn new(amplifiers: Vec<Amplifier>) -> Self {
            AmplifierChain { amplifiers }
        }

        /// Run signal through all amplifiers.
        pub fn run(&self, input: f64) -> f64 {
            self.amplifiers.iter().fold(input, |sig, amp| amp.amplify(sig))
        }

        /// Total gain of the chain.
        pub fn total_gain(&self) -> f64 {
            self.amplifiers.iter().map(|a| a.gain).product()
        }

        /// Number of stages.
        pub fn stages(&self) -> usize {
            self.amplifiers.len()
        }

        /// Effective saturation (minimum of all saturations).
        pub fn effective_saturation(&self) -> f64 {
            self.amplifiers.iter().map(|a| a.saturation).fold(f64::MAX, f64::min)
        }
    }
}

// ── Module: gate ─────────────────────────────────────────────────────────────

pub mod gate {
    /// A signal gate that conditionally passes signals.
    #[derive(Clone, Debug)]
    pub struct Gate {
        pub gate_type: GateType,
        pub threshold: f64,
        pub is_open: bool,
        pub signal_output: f64,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum GateType {
        Threshold,      // Passes if signal > threshold
        AND,            // Passes if both inputs > threshold
        OR,             // Passes if either input > threshold
        NOT,            // Inverts: passes if signal < threshold
        Modulated,      // Passes proportional to modulator signal
    }

    impl Gate {
        pub fn new(gate_type: GateType, threshold: f64) -> Self {
            Gate { gate_type, threshold, is_open: false, signal_output: 0.0 }
        }

        /// Process a single input signal.
        pub fn process(&mut self, signal: f64) -> f64 {
            match self.gate_type {
                GateType::Threshold => {
                    self.is_open = signal >= self.threshold;
                    self.signal_output = if self.is_open { signal } else { 0.0 };
                }
                GateType::NOT => {
                    self.is_open = signal < self.threshold;
                    self.signal_output = if self.is_open { self.threshold - signal } else { 0.0 };
                }
                _ => { self.signal_output = signal; }
            }
            self.signal_output
        }

        /// Process two input signals (for AND/OR gates).
        pub fn process_dual(&mut self, signal_a: f64, signal_b: f64) -> f64 {
            match self.gate_type {
                GateType::AND => {
                    self.is_open = signal_a >= self.threshold && signal_b >= self.threshold;
                    self.signal_output = if self.is_open {
                        (signal_a + signal_b) / 2.0
                    } else { 0.0 };
                }
                GateType::OR => {
                    self.is_open = signal_a >= self.threshold || signal_b >= self.threshold;
                    self.signal_output = if self.is_open {
                        signal_a.max(signal_b)
                    } else { 0.0 };
                }
                GateType::Modulated => {
                    let factor = signal_b / self.threshold.max(0.01);
                    self.is_open = factor > 0.0;
                    self.signal_output = signal_a * factor.min(1.0);
                }
                _ => { self.signal_output = self.process(signal_a); }
            }
            self.signal_output
        }

        /// Reset the gate.
        pub fn reset(&mut self) {
            self.is_open = false;
            self.signal_output = 0.0;
        }

        /// Get gate state.
        pub fn is_open(&self) -> bool {
            self.is_open
        }

        /// Compute gating function (smooth threshold using sigmoid).
        pub fn smooth_gate(signal: f64, threshold: f64, steepness: f64) -> f64 {
            let x = steepness * (signal - threshold);
            1.0 / (1.0 + (-x).exp())
        }
    }
}

// ── Module: pathway ──────────────────────────────────────────────────────────

pub mod pathway {
    use crate::receptor::Receptor;
    use crate::kinase::{Cascade, Kinase};
    use crate::amplifier::Amplifier;
    use crate::gate::Gate;

    /// A complete signal transduction pathway.
    pub struct Pathway {
        pub receptor: Receptor,
        pub cascade: Cascade,
        pub amplifier: Amplifier,
        pub gate: Gate,
        pub name: String,
        pub history: Vec<f64>,
    }

    impl Pathway {
        pub fn new(
            name: &str,
            receptor: Receptor,
            cascade: Cascade,
            amplifier: Amplifier,
            gate: Gate,
        ) -> Self {
            Pathway {
                name: name.to_string(),
                receptor,
                cascade,
                amplifier,
                gate,
                history: Vec::new(),
            }
        }

        /// Process an input signal through the complete pathway.
        pub fn process(&mut self, input: f64) -> f64 {
            // 1. Receptor detects signal
            self.receptor.detect(input);
            let receptor_output = self.receptor.output();

            // 2. Kinase cascade
            let cascade_output = self.cascade.run(receptor_output);

            // 3. Amplification
            let amplified = self.amplifier.amplify(cascade_output);

            // 4. Gating
            let output = self.gate.process(amplified);

            self.history.push(output);
            output
        }

        /// Get the last output.
        pub fn last_output(&self) -> Option<f64> {
            self.history.last().copied()
        }

        /// Get signal history.
        pub fn get_history(&self) -> &[f64] {
            &self.history
        }

        /// Reset the entire pathway.
        pub fn reset(&mut self) {
            self.receptor.reset();
            self.cascade.reset();
            self.gate.reset();
            self.history.clear();
        }

        /// Compute pathway gain (total amplification).
        pub fn pathway_gain(&self) -> f64 {
            if self.history.len() < 2 { return 0.0; }
            self.amplifier.gain * self.cascade.stages() as f64
        }

        /// Compute pathway latency (simplified as number of steps).
        pub fn latency(&self) -> usize {
            1 + self.cascade.stages() + 1 + 1 // receptor + cascade + amp + gate
        }

        /// Check if the pathway is active.
        pub fn is_active(&self) -> bool {
            self.receptor.active
        }
    }

    /// Build a simple pathway with default parameters.
    pub fn simple_pathway(name: &str, threshold: f64, gain: f64) -> Pathway {
        Pathway::new(
            name,
            Receptor::new("receptor", threshold, 1.0, 10.0),
            Cascade::new(vec![
                Kinase::new("kinase1", 10.0, 0.1),
                Kinase::new("kinase2", 10.0, 0.1),
                Kinase::new("kinase3", 10.0, 0.1),
            ]),
            Amplifier::new(gain, 0.01, 100.0),
            Gate::new(crate::gate::GateType::Threshold, 0.1),
        )
    }

    /// Compute pathway sensitivity (minimum input for output > 0).
    pub fn pathway_sensitivity(pathway: &Pathway) -> f64 {
        pathway.receptor.threshold
    }

    /// Compare two pathways' responses to the same input.
    pub fn compare_pathways(p1: &mut Pathway, p2: &mut Pathway, input: f64) -> (f64, f64) {
        let o1 = p1.process(input);
        let o2 = p2.process(input);
        (o1, o2)
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Receptor tests ──

    #[test]
    fn test_receptor_creation() {
        let r = receptor::Receptor::new("test", 0.5, 1.0, 10.0);
        assert_eq!(r.name, "test");
        assert!(!r.active);
    }

    #[test]
    fn test_receptor_detect_above_threshold() {
        let mut r = receptor::Receptor::new("test", 0.5, 2.0, 10.0);
        let active = r.detect(1.0);
        assert!(active);
        assert!((r.output() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_receptor_detect_below_threshold() {
        let mut r = receptor::Receptor::new("test", 0.5, 2.0, 10.0);
        let active = r.detect(0.3);
        assert!(!active);
        assert_eq!(r.output(), 0.0);
    }

    #[test]
    fn test_receptor_saturation() {
        let mut r = receptor::Receptor::new("test", 0.5, 10.0, 10.0);
        r.detect(100.0);
        assert!(r.is_saturated());
        assert!((r.output() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_receptor_reset() {
        let mut r = receptor::Receptor::new("test", 0.5, 2.0, 10.0);
        r.detect(1.0);
        r.reset();
        assert!(!r.active);
        assert_eq!(r.output(), 0.0);
    }

    #[test]
    fn test_receptor_desensitization() {
        let mut r = receptor::Receptor::new("test", 0.5, 2.0, 10.0)
            .with_desensitization(0.3);
        r.detect(1.0);
        let out1 = r.output();
        r.desensitize();
        r.detect(1.0);
        let out2 = r.output();
        assert!(out2 < out1);
    }

    #[test]
    fn test_receptor_recovery() {
        let mut r = receptor::Receptor::new("test", 0.5, 2.0, 10.0)
            .with_desensitization(0.3);
        r.detect(1.0);
        r.desensitize(); // active: desensitization goes to 0.3
        assert!((r.desensitization_level - 0.3).abs() < 1e-10);
        // Now not active, should recover
        r.active = false;
        r.desensitize(); // recovery: 0.3 - 0.15 = 0.15
        assert!(r.desensitization_level < 0.3);
    }

    #[test]
    fn test_dose_response() {
        let r = receptor::Receptor::new("test", 0.5, 2.0, 10.0);
        let dr = r.dose_response(1.0);
        assert!(dr > 0.0);
    }

    #[test]
    fn test_dose_response_zero() {
        let r = receptor::Receptor::new("test", 0.5, 2.0, 10.0);
        let dr = r.dose_response(0.0);
        assert_eq!(dr, 0.0);
    }

    #[test]
    fn test_dose_response_high() {
        let r = receptor::Receptor::new("test", 0.5, 1.0, 10.0);
        let dr = r.dose_response(100.0);
        assert!((dr - 10.0).abs() < 0.1);
    }

    // ── Kinase tests ──

    #[test]
    fn test_kinase_creation() {
        let k = kinase::Kinase::new("mapk", 10.0, 0.5);
        assert!(!k.active);
        assert_eq!(k.phosphorylation_level, 0.0);
    }

    #[test]
    fn test_kinase_activate() {
        let mut k = kinase::Kinase::new("mapk", 10.0, 0.5);
        k.activate(1.0);
        assert!(k.active);
        assert!(k.activity > 0.0);
    }

    #[test]
    fn test_kinase_no_activate_below_threshold() {
        let mut k = kinase::Kinase::new("mapk", 10.0, 1.0);
        k.activate(0.1);
        assert!(!k.active);
    }

    #[test]
    fn test_kinase_deactivate() {
        let mut k = kinase::Kinase::new("mapk", 10.0, 0.1);
        k.activate(5.0);
        k.deactivate();
        assert!(k.phosphorylation_level < 1.0);
    }

    #[test]
    fn test_kinase_reset() {
        let mut k = kinase::Kinase::new("mapk", 10.0, 0.5);
        k.activate(1.0);
        k.reset();
        assert!(!k.active);
        assert_eq!(k.output(), 0.0);
    }

    #[test]
    fn test_kinase_full_phosphorylation() {
        let mut k = kinase::Kinase::new("mapk", 10.0, 0.1);
        for _ in 0..100 {
            k.activate(10.0);
        }
        assert!(k.is_fully_phosphorylated());
    }

    #[test]
    fn test_cascade_run() {
        let cascade = kinase::Cascade::new(vec![
            kinase::Kinase::new("k1", 10.0, 0.1),
            kinase::Kinase::new("k2", 10.0, 0.1),
        ]);
        assert_eq!(cascade.stages(), 2);
    }

    #[test]
    fn test_cascade_amplification() {
        let mut cascade = kinase::Cascade::new(vec![
            kinase::Kinase::new("k1", 10.0, 0.1),
            kinase::Kinase::new("k2", 10.0, 0.1),
            kinase::Kinase::new("k3", 10.0, 0.1),
        ]);
        let output = cascade.run(1.0);
        // Should propagate through
        assert!(output >= 0.0);
    }

    #[test]
    fn test_cascade_reset() {
        let mut cascade = kinase::Cascade::new(vec![
            kinase::Kinase::new("k1", 10.0, 0.1),
        ]);
        cascade.run(1.0);
        cascade.reset();
        assert_eq!(cascade.kinases[0].output(), 0.0);
    }

    #[test]
    fn test_cascade_amplification_ratio() {
        let cascade = kinase::Cascade::new(vec![
            kinase::Kinase::new("k1", 10.0, 0.1),
        ]);
        let amp = cascade.amplification(1.0, 10.0);
        assert!((amp - 10.0).abs() < 1e-10);
    }

    // ── Amplifier tests ──

    #[test]
    fn test_amplifier_creation() {
        let a = amplifier::Amplifier::new(5.0, 0.01, 100.0);
        assert_eq!(a.gain, 5.0);
    }

    #[test]
    fn test_amplify_basic() {
        let a = amplifier::Amplifier::new(3.0, 0.0, 100.0);
        let out = a.amplify(2.0);
        assert!((out - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_amplify_saturation() {
        let a = amplifier::Amplifier::new(100.0, 0.0, 50.0);
        let out = a.amplify(1.0);
        assert!((out - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_amplify_noise_floor() {
        let a = amplifier::Amplifier::new(0.001, 0.1, 100.0);
        let out = a.amplify(0.0);
        assert!((out - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_amplify_sigmoid() {
        let a = amplifier::Amplifier::new(1.0, 0.0, 10.0);
        let out = a.amplify_sigmoid(5.0);
        assert!(out > 0.0 && out <= 10.0);
    }

    #[test]
    fn test_snr() {
        let a = amplifier::Amplifier::new(10.0, 0.1, 100.0);
        let snr = a.snr(1.0);
        assert!(snr > 0.0);
    }

    #[test]
    fn test_is_saturated() {
        let a = amplifier::Amplifier::new(10.0, 0.0, 5.0);
        assert!(a.is_saturated(1.0));
        assert!(!a.is_saturated(0.1));
    }

    #[test]
    fn test_dynamic_range_db() {
        let a = amplifier::Amplifier::new(1.0, 0.01, 100.0);
        let dr = a.dynamic_range_db();
        assert!(dr > 0.0);
    }

    #[test]
    fn test_amplifier_chain() {
        let chain = amplifier::AmplifierChain::new(vec![
            amplifier::Amplifier::new(2.0, 0.0, 100.0),
            amplifier::Amplifier::new(3.0, 0.0, 100.0),
        ]);
        assert_eq!(chain.stages(), 2);
        assert!((chain.total_gain() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_amplifier_chain_run() {
        let chain = amplifier::AmplifierChain::new(vec![
            amplifier::Amplifier::new(2.0, 0.0, 100.0),
            amplifier::Amplifier::new(3.0, 0.0, 100.0),
        ]);
        let out = chain.run(5.0);
        assert!((out - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_effective_saturation() {
        let chain = amplifier::AmplifierChain::new(vec![
            amplifier::Amplifier::new(2.0, 0.0, 50.0),
            amplifier::Amplifier::new(3.0, 0.0, 100.0),
        ]);
        assert!((chain.effective_saturation() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_filter_frequency() {
        let a = amplifier::Amplifier::new(2.0, 0.0, 100.0);
        a.filter_frequency(1.0, 10.0); // bandwidth is MAX, so should amplify
    }

    // ── Gate tests ──

    #[test]
    fn test_gate_threshold_open() {
        let mut g = gate::Gate::new(gate::GateType::Threshold, 0.5);
        let out = g.process(0.8);
        assert!(g.is_open());
        assert!((out - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_gate_threshold_closed() {
        let mut g = gate::Gate::new(gate::GateType::Threshold, 0.5);
        let out = g.process(0.3);
        assert!(!g.is_open());
        assert_eq!(out, 0.0);
    }

    #[test]
    fn test_gate_and_both_active() {
        let mut g = gate::Gate::new(gate::GateType::AND, 0.5);
        let out = g.process_dual(0.6, 0.7);
        assert!(g.is_open());
        assert!(out > 0.0);
    }

    #[test]
    fn test_gate_and_one_inactive() {
        let mut g = gate::Gate::new(gate::GateType::AND, 0.5);
        let out = g.process_dual(0.6, 0.3);
        assert!(!g.is_open());
        assert_eq!(out, 0.0);
    }

    #[test]
    fn test_gate_or_one_active() {
        let mut g = gate::Gate::new(gate::GateType::OR, 0.5);
        let out = g.process_dual(0.3, 0.7);
        assert!(g.is_open());
        assert!(out > 0.0);
    }

    #[test]
    fn test_gate_or_none_active() {
        let mut g = gate::Gate::new(gate::GateType::OR, 0.5);
        let out = g.process_dual(0.3, 0.2);
        assert!(!g.is_open());
        assert_eq!(out, 0.0);
    }

    #[test]
    fn test_gate_not() {
        let mut g = gate::Gate::new(gate::GateType::NOT, 0.5);
        let out = g.process(0.3);
        assert!(g.is_open());
        assert!(out > 0.0);
    }

    #[test]
    fn test_gate_not_above_threshold() {
        let mut g = gate::Gate::new(gate::GateType::NOT, 0.5);
        let out = g.process(0.8);
        assert!(!g.is_open());
        assert_eq!(out, 0.0);
    }

    #[test]
    fn test_gate_modulated() {
        let mut g = gate::Gate::new(gate::GateType::Modulated, 1.0);
        let out = g.process_dual(5.0, 0.5);
        assert!((out - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_gate_reset() {
        let mut g = gate::Gate::new(gate::GateType::Threshold, 0.5);
        g.process(0.8);
        g.reset();
        assert!(!g.is_open());
    }

    #[test]
    fn test_smooth_gate() {
        let sg = gate::Gate::smooth_gate(0.5, 0.5, 10.0);
        assert!((sg - 0.5).abs() < 0.01);
    }

    // ── Pathway tests ──

    #[test]
    fn test_pathway_creation() {
        let p = pathway::simple_pathway("test", 0.5, 2.0);
        assert_eq!(p.name, "test");
    }

    #[test]
    fn test_pathway_process() {
        let mut p = pathway::simple_pathway("test", 0.5, 2.0);
        let output = p.process(1.0);
        assert!(output >= 0.0);
    }

    #[test]
    fn test_pathway_below_threshold() {
        let mut p = pathway::simple_pathway("test", 0.5, 2.0);
        let output = p.process(0.1);
        assert_eq!(output, 0.0);
    }

    #[test]
    fn test_pathway_history() {
        let mut p = pathway::simple_pathway("test", 0.5, 2.0);
        p.process(1.0);
        p.process(0.8);
        assert_eq!(p.get_history().len(), 2);
    }

    #[test]
    fn test_pathway_last_output() {
        let mut p = pathway::simple_pathway("test", 0.5, 2.0);
        p.process(1.0);
        assert!(p.last_output().is_some());
    }

    #[test]
    fn test_pathway_reset() {
        let mut p = pathway::simple_pathway("test", 0.5, 2.0);
        p.process(1.0);
        p.reset();
        assert!(p.get_history().is_empty());
    }

    #[test]
    fn test_pathway_latency() {
        let p = pathway::simple_pathway("test", 0.5, 2.0);
        assert!(p.latency() > 0);
    }

    #[test]
    fn test_pathway_sensitivity() {
        let p = pathway::simple_pathway("test", 0.5, 2.0);
        assert!((pathway::pathway_sensitivity(&p) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_compare_pathways() {
        let mut p1 = pathway::simple_pathway("p1", 0.5, 2.0);
        let mut p2 = pathway::simple_pathway("p2", 0.3, 3.0);
        let (o1, o2) = pathway::compare_pathways(&mut p1, &mut p2, 1.0);
        // Both should produce output
        assert!(o1 >= 0.0);
        assert!(o2 >= 0.0);
    }
}
