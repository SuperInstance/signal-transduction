# signal-transduction

> **How information flows through the system. From signal to response.**

[![crates.io](https://img.shields.io/crates/v/signal-transduction.svg)](https://crates.io/crates/signal-transduction)
[![docs.rs](https://docs.rs/signal-transduction/badge.svg)](https://docs.rs/signal-transduction)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library modeling biological signal transduction pathways for agent systems. Implements receptors with dose-response curves, kinase cascades with amplification, signal gating (AND/OR/NOT logic), multi-stage pathways with feedback, and complete cascade processing. Maps cellular biology's elegant information processing to AI agent architectures.

---

## Table of Contents

- [What is Signal Transduction?](#what-is-signal-transduction)
- [Why Does This Matter?](#why-does-this-matter)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Mathematical Background](#mathematical-background)
- [Installation](#installation)
- [Related Crates](#related-crates)
- [License](#license)

---

## What is Signal Transduction?

In cellular biology, **signal transduction** is how a cell converts an external signal (hormone, neurotransmitter, light) into an internal response (gene expression, movement, secretion). The process involves:

1. **Receptor binding** — an external molecule docks on a cell surface receptor
2. **Conformational change** — the receptor switches from inactive to active
3. **Kinase cascade** — a chain of enzymes, each activating the next, amplifying the signal
4. **Signal integration** — multiple pathways converge on shared targets
5. **Feedback regulation** — the system adapts to sustained stimulation (desensitization)

This library maps each biological component to an agent-system equivalent:

```
Biological                    Agent System
──────────                    ────────────
External signal        →      Incoming event / message
Receptor               →      Event detector with threshold
Kinase cascade         →      Processing pipeline with amplification
Transcription factor   →      Action trigger / state change
Desensitization        →      Attention fatigue / habituation
Feedback loop          →      Adaptive sensitivity tuning
```

The key insight: cells have solved the problem of processing noisy, partial signals into reliable decisions. We can learn from 4 billion years of optimization.

## Why Does This Matter?

**For event-driven architectures**: Signal cascades provide a natural pattern for processing external events through amplification, filtering, and integration stages — more robust than simple callback chains.

**For attention mechanisms**: Receptor desensitization models attention fatigue — an agent that's been exposed to the same signal repeatedly should respond less strongly, freeing resources for novel inputs.

**For multi-signal integration**: AND/OR/NOT gates allow agents to make decisions based on combinations of signals, just as cells integrate multiple environmental cues before committing to a response.

**For adaptive systems**: Dose-response curves and amplification chains create nonlinear input-output mappings that can be tuned for sensitivity, dynamic range, and saturation — the same engineering problems cells solve.

## Architecture

```
signal-transduction
│
├── receptor module            ← Signal detection
│   ├── Receptor                   Threshold detector with sensitivity
│   ├── detect(input)              Activate if input ≥ threshold
│   ├── dose_response(conc)        Hill equation response
│   ├── desensitize()              Reduce sensitivity on repeated activation
│   └── is_saturated()             At maximum output?
│
├── kinase module              ← Signal amplification
│   ├── Kinase                     Enzyme with phosphorylation levels
│   ├── activate(signal)           Receive upstream signal
│   ├── cascade_step(upstream)     Process and forward
│   └── Cascade                    Chain of kinases
│       ├── run(input)             Full cascade amplification
│       └── amplification(in, out) Gain ratio
│
├── amplifier module           ← Signal conditioning
│   ├── Amplifier                  Gain + saturation + noise floor
│   ├── amplify(input)             Linear amplification
│   ├── amplify_sigmoid(input)     S-shaped (soft saturation)
│   ├── snr(signal)                Signal-to-noise ratio
│   └── AmplifierChain             Series of amplifiers
│       ├── run(input)             Multi-stage amplification
│       └── total_gain()           Combined gain factor
│
├── gate module                ← Signal logic
│   ├── Gate + GateType            AND / OR / NOT / CUSTOM
│   ├── process(signal)            Single-input gating
│   ├── process_dual(a, b)         Two-input gating
│   └── smooth_gate(s, th, k)      Soft sigmoidal gating
│
└── pathway module             ← End-to-end processing
    ├── Pathway                    Receptor → Cascade → Amplifier → Output
    ├── process(input)             Run full pathway
    ├── pathway_gain()             Total input→output ratio
    ├── latency()                  Processing stages (time delay)
    └── simple_pathway()           Convenience constructor
```

## Quick Start

```rust
use signal_transduction::{
    receptor::Receptor,
    kinase::{Kinase, Cascade},
    amplifier::Amplifier,
    gate::{Gate, GateType},
    pathway::Pathway,
};

// Create a receptor that detects signals above threshold 0.5
let mut receptor = Receptor::new("temperature", 0.5, 2.0, 10.0)
    .with_desensitization(0.1); // adapts to repeated stimulation

let detected = receptor.detect(0.7); // true: 0.7 > 0.5
let output = receptor.output();       // 1.4 = 0.7 × 2.0 sensitivity
println!("Detected: {}, Output: {:.2}", detected, output);

// Dose-response curve (Hill equation)
let response = receptor.dose_response(1.0); // nonlinear response
println!("Dose response at 1.0: {:.4}", response);

// Build a kinase cascade (3-stage amplification)
let kinases = vec![
    Kinase::new("RAS",  1.0, 0.1),
    Kinase::new("RAF",  1.0, 0.1),
    Kinase::new("MEK",  1.0, 0.1),
];
let mut cascade = Cascade::new(kinases);
let amplified = cascade.run(0.5);
println!("Cascade output: {:.4}", amplified);

// Signal gating (AND gate)
let mut gate = Gate::new(GateType::And, 0.5);
let result = gate.process_dual(0.7, 0.8); // both above threshold → pass
println!("AND gate output: {:.4}", result);

// Complete pathway
let mut pathway = Pathway::new(
    "danger_response",
    Receptor::new("threat", 0.3, 3.0, 10.0),
    Cascade::new(vec![
        Kinase::new("K1", 2.0, 0.1),
        Kinase::new("K2", 2.0, 0.1),
    ]),
    Amplifier::new(1.5, 0.01, 20.0),
);
let final_output = pathway.process(0.4);
println!("Pathway output: {:.4}", final_output);
```

## API Reference

### Receptor

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name, threshold, sensitivity, max)` | `Self` | Create receptor |
| `detect(input)` | `bool` | Activate if input ≥ threshold |
| `output()` | `f64` | Current output signal |
| `dose_response(concentration)` | `f64` | Hill equation response |
| `desensitize()` | `()` | Reduce sensitivity (adaptation) |
| `is_saturated()` | `bool` | At maximum output? |
| `reset()` | `()` | Clear activation state |
| `with_desensitization(rate)` | `Self` | Builder: set adaptation rate |

### Kinase & Cascade

| Method | Returns | Description |
|--------|---------|-------------|
| `Kinase::new(name, max_activity, threshold)` | `Self` | Create kinase |
| `k.activate(signal)` | `()` | Receive upstream signal |
| `k.cascade_step(upstream)` | `f64` | Process and return output |
| `k.is_fully_phosphorylated()` | `bool` | Maximum activation |
| `Cascade::new(kinases)` | `Self` | Create cascade chain |
| `c.run(input)` | `f64` | Run full cascade |
| `c.amplification(in, out)` | `f64` | Gain ratio |
| `c.stages()` | `usize` | Number of kinase stages |

### Amplifier

| Method | Returns | Description |
|--------|---------|-------------|
| `new(gain, noise_floor, saturation)` | `Self` | Create amplifier |
| `amplify(input)` | `f64` | Linear amplification |
| `amplify_sigmoid(input)` | `f64` | Soft-saturating amplification |
| `snr(signal)` | `f64` | Signal-to-noise ratio |
| `dynamic_range_db()` | `f64` | Usable range in decibels |
| `AmplifierChain::run(input)` | `f64` | Multi-stage output |

### Gate

| Method | Returns | Description |
|--------|---------|-------------|
| `new(GateType, threshold)` | `Self` | Create logic gate |
| `process(signal)` | `f64` | Single-input gating |
| `process_dual(a, b)` | `f64` | Two-input logic |
| `smooth_gate(s, th, steepness)` | `f64` | Sigmoidal soft gating |
| `is_open()` | `bool` | Current gate state |

### Pathway

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name, receptor, cascade, amplifier)` | `Self` | Create full pathway |
| `process(input)` | `f64` | End-to-end processing |
| `pathway_gain()` | `f64` | Total amplification |
| `latency()` | `usize` | Processing stages |
| `is_active()` | `bool` | Currently processing |
| `simple_pathway(name, threshold, gain)` | `Pathway` | Convenience constructor |

## Mathematical Background

### Hill Equation (Dose-Response)

The receptor's dose-response follows the Hill equation:

```
R = V_max × c^n / (K_d^n + c^n)
```

Where c is the signal concentration, K_d is the half-maximal concentration, and n is the Hill coefficient (cooperativity). This produces the characteristic sigmoidal response curve — low sensitivity at low concentrations, rapid response in the dynamic range, and saturation at high concentrations.

### Kinase Cascade Amplification

Each kinase in a cascade amplifies the signal:

```
K_i.output = K_i.max_activity × σ(K_i.input − K_i.threshold)
```

Where σ is a soft step function. A cascade of n kinases produces multiplicative amplification:

```
Total gain = Π_i gain_i ≈ (max_activity)^n
```

This explains how a single photon (one molecule of activated rhodopsin) can activate millions of molecules in the visual transduction cascade.

### Signal-to-Noise Ratio

```
SNR(dB) = 10 × log₁₀(signal² / noise_floor²)
```

The dynamic range of a pathway is the range of input signals where SNR is acceptable — above the noise floor but below saturation.

### Smooth Gating

The smooth gate function uses a sigmoidal activation:

```
g(s) = 1 / (1 + e^{-k(s - θ)})
```

Where k controls the steepness (sharpness of the threshold) and θ is the threshold. As k → ∞, this approaches a step function.

## Installation

```bash
cargo add signal-transduction
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
signal-transduction = "0.1"
```

## Related Crates

Part of the **SuperInstance Exocortex** ecosystem:

- **[markov-blanket](https://github.com/SuperInstance/markov-blanket)** — Statistical boundary between agent and world
- **[free-energy](https://github.com/SuperInstance/free-energy)** — Variational free energy computation
- **[active-inference](https://github.com/SuperInstance/active-inference)** — Action as surprise minimization
- **[morphogenesis](https://github.com/SuperInstance/morphogenesis)** — Turing patterns for agent development
- **[cortex-bus-protocol](https://github.com/SuperInstance/cortex-bus-protocol)** — CQRS event bus for agent messaging

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project.
