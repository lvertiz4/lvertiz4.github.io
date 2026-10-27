//! Browser demo for the saga executor.
//!
//! Phase 1 of the project is `saga-core`: a sans-I/O state machine with no
//! UI and no I/O of its own. This crate is the thin wasm wrapper the project
//! page will load. It plays the runtime's role (ask the machine for a
//! command, pretend to perform it, fold the resulting event back in) and
//! returns the durable log as text, so the page can show exactly what the
//! executor would have written to disk.
//!
//! Nothing here touches the DOM. Rendering is the page's job; this crate
//! exposes one function to JavaScript and keeps its dependencies minimal.

use saga_core::{Command, ErrorRecord, NodeId, Output, SagaEvent, SagaId, SagaPlan, SagaState};
use wasm_bindgen::prelude::wasm_bindgen;

/// Runs a linear saga of `steps` steps and returns its event log, one event
/// per line, followed by the final outcome.
///
/// If `fail_at` names a step, that step fails and the executor unwinds every
/// step before it in reverse. From JavaScript, pass `undefined` for no failure.
#[wasm_bindgen]
pub fn run_linear_saga(steps: u32, fail_at: Option<u32>) -> String {
    let plan = match SagaPlan::linear(SagaId(1), steps) {
        Ok(plan) => plan,
        Err(err) => return format!("invalid plan: {err:?}"),
    };
    let fail_at = fail_at.map(NodeId);

    let mut state = SagaState::new(&plan);
    let mut log = String::new();
    // Bounded, so a wedged machine returns what it has instead of hanging the tab.
    for _ in 0..10_000 {
        let event = match state.next_command() {
            Command::Idle => break,
            // Events the core originates itself (Started, StepStarted, Finished, ...).
            Command::Append(event) => event,
            // The runtime would run the real action here; we script its result.
            Command::RunAction { node } => {
                if fail_at == Some(node) {
                    SagaEvent::StepFailed {
                        node,
                        error: ErrorRecord::new("injected failure"),
                    }
                } else {
                    SagaEvent::StepDone {
                        node,
                        output: Output(node.0.to_le_bytes().to_vec()),
                    }
                }
            }
            // Compensations always succeed in this demo.
            Command::RunUndo { node } => SagaEvent::UndoDone { node },
        };
        log.push_str(&format!("{event:?}\n"));
        state = state.apply(&event);
    }
    log.push_str(&format!("outcome: {:?}", state.outcome()));
    log
}

#[cfg(test)]
mod tests {
    use super::run_linear_saga;

    #[test]
    fn happy_path_completes() {
        let log = run_linear_saga(3, None);
        assert!(log.contains("StepDone { node: NodeId(2)"), "{log}");
        assert!(log.ends_with("outcome: Some(Completed)"), "{log}");
    }

    #[test]
    fn failure_unwinds_earlier_steps_in_reverse() {
        let log = run_linear_saga(3, Some(2));
        let undo_1 = log.find("UndoDone { node: NodeId(1) }").expect("step 1 undone");
        let undo_0 = log.find("UndoDone { node: NodeId(0) }").expect("step 0 undone");
        assert!(undo_1 < undo_0, "undo must run in reverse order:\n{log}");
        assert!(log.ends_with("outcome: Some(Unwound)"), "{log}");
    }

    #[test]
    fn empty_plan_is_reported_not_panicked() {
        assert!(run_linear_saga(0, None).starts_with("invalid plan"));
    }
}
