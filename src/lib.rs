//! Check an openBIM IFC model against Indonesian SNI requirements.
//!
//! # Status
//!
//! Early. The scaffolding, the rule interface and the reporting shape are
//! here; the IFC parser and the rules themselves are not. See the open issues
//! for what is planned and in what order.
//!
//! # What this is for
//!
//! A BIM model carries geometry and properties. Whether that model satisfies
//! a building standard is a separate question, and today it is answered by a
//! person reading a drawing set. This crate exists to answer the mechanical
//! part of that question automatically, and to be explicit about the part it
//! cannot answer.
//!
//! # Not a substitute for an engineer
//!
//! A passing report means the rules implemented here found nothing wrong. It
//! does not mean the design complies with SNI, and it does not mean the
//! building is safe. Rules cover what can be checked mechanically from model
//! data. Engineering judgement, load paths, constructability and everything
//! the model does not record are outside what any tool of this kind can see.
//!
//! A rule that cannot evaluate an element reports [`Outcome::NotEvaluated`]
//! with a reason, never a pass. Silence is not compliance.

#![forbid(unsafe_code)]

/// What a rule concluded about one element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The element satisfies the rule.
    Pass,
    /// The element violates the rule, with an explanation an engineer can act on.
    Fail {
        /// What is wrong, in terms of the requirement rather than the code.
        detail: String,
    },
    /// The rule could not be evaluated. Never treat this as a pass.
    NotEvaluated {
        /// Why, for example a property the model does not carry.
        reason: String,
    },
}

/// A single finding: one rule, applied to one element.
#[derive(Debug, Clone)]
pub struct Finding {
    /// The rule that produced this, for example `SNI-2847-9.3.3.1`.
    pub rule_id: String,
    /// The IFC GlobalId of the element the rule looked at.
    pub element_id: String,
    /// What the rule concluded.
    pub outcome: Outcome,
    /// The clause the requirement comes from, so a reader can check the source.
    pub clause: String,
}

/// The result of checking a model.
#[derive(Debug, Clone, Default)]
pub struct Report {
    /// Every finding, in the order the rules ran.
    pub findings: Vec<Finding>,
}

impl Report {
    /// Findings that failed.
    pub fn failures(&self) -> impl Iterator<Item = &Finding> {
        self.findings
            .iter()
            .filter(|f| matches!(f.outcome, Outcome::Fail { .. }))
    }

    /// Findings a rule could not evaluate. Worth reading before trusting a
    /// report with no failures, since an unevaluated model fails nothing.
    pub fn not_evaluated(&self) -> impl Iterator<Item = &Finding> {
        self.findings
            .iter()
            .filter(|f| matches!(f.outcome, Outcome::NotEvaluated { .. }))
    }

    /// True when nothing failed. Read [`Report::not_evaluated`] as well: a
    /// report can be clean because every rule was skipped.
    pub fn is_clean(&self) -> bool {
        self.failures().next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(outcome: Outcome) -> Finding {
        Finding {
            rule_id: "SNI-TEST-1".into(),
            element_id: "0abc".into(),
            outcome,
            clause: "cl. 1.1".into(),
        }
    }

    #[test]
    fn a_report_with_no_findings_is_clean() {
        assert!(Report::default().is_clean());
    }

    #[test]
    fn a_failure_makes_a_report_unclean() {
        let r = Report {
            findings: vec![finding(Outcome::Fail {
                detail: "cover below the minimum".into(),
            })],
        };
        assert!(!r.is_clean());
        assert_eq!(r.failures().count(), 1);
    }

    #[test]
    fn an_unevaluated_rule_is_not_a_failure_and_not_a_pass() {
        let r = Report {
            findings: vec![finding(Outcome::NotEvaluated {
                reason: "the model carries no cover property".into(),
            })],
        };
        // Clean, because nothing failed. This is exactly why not_evaluated
        // has to be read alongside is_clean rather than instead of it.
        assert!(r.is_clean());
        assert_eq!(r.not_evaluated().count(), 1);
        assert_eq!(r.failures().count(), 0);
    }
}
