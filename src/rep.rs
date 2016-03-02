//! Rust representations of Github API data structures

use super::{SortDirection, Error, State as StdState};
use super::issues::Sort;
use std::collections::HashMap;
use std::hash::Hash;
use std::option::Option;
use url::form_urlencoded;

extern crate serializable_enum;
extern crate serde;
extern crate serde_json;

include!(concat!(env!("OUT_DIR"), "/rep.rs"));

serializable_enum! {
    /// representation of deployment and commit status states
    #[derive(Clone, Debug, PartialEq)]
    pub enum StatusState {
        /// pending
        Pending,
        /// success
        Success,
        /// error
        Error,
        /// failure
        Failure,
    }
    StatusStateVisitor
}

impl_as_ref_from_str! {
    StatusState {
        Pending => "pending",
        Success => "success",
        Error => "error",
        Failure => "failure",
    }
    Error::Parse
}

impl Default for StatusState {
    fn default() -> StatusState {
        StatusState::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::super::State as StdState;
    use serde::ser::Serialize;
    use std::collections::HashMap;
    use super::*;
    use super::serde_json;

    fn test_encoding<E: Serialize>(tests: Vec<(E, &str)>) {
        for test in tests {
            match test {
                (k, v) => assert_eq!(serde_json::to_string(&k).unwrap(), v),
            }
        }
    }

    #[test]
    fn gist_reqs() {
        let mut files = HashMap::new();
        files.insert("foo", "bar");
        let tests = vec![
            (
                GistOptions::new(None as Option<String>, true, files.clone()),
                r#"{"public":true,"files":{"foo":{"content":"bar"}}}"#
            ),
            (
                GistOptions::new(Some("desc"), true, files.clone()),
                r#"{"description":"desc","public":true,"files":{"foo":{"content":"bar"}}}"#
            )
        ];
        test_encoding(tests);
    }

    #[test]
    fn deserialize_status_state() {
        for (json, expect) in vec![
            ("\"pending\"", StatusState::Pending),
            ("\"success\"", StatusState::Success),
            ("\"error\"", StatusState::Error),
            ("\"failure\"", StatusState::Failure)
        ] {
            assert_eq!(serde_json::from_str::<StatusState>(json).unwrap(), expect)
        }
    }

    #[test]
    fn deployment_reqs() {
        let tests = vec![(DeploymentOptions::builder("test").build(),
                          r#"{"ref":"test"}"#),
                         (DeploymentOptions::builder("test").task("launchit").build(),
                          r#"{"ref":"test","task":"launchit"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn deployment_status_reqs() {
        let tests = vec![
            (
                DeploymentStatusOptions::builder(StatusState::Pending).build(),
                r#"{"state":"pending"}"#
            ),
            (
                DeploymentStatusOptions::builder(StatusState::Pending).target_url("http://host.com").build(),
                r#"{"state":"pending","target_url":"http://host.com"}"#
            ),
            (
                DeploymentStatusOptions::builder(StatusState::Pending).target_url("http://host.com").description("desc").build(),
                r#"{"state":"pending","target_url":"http://host.com","description":"desc"}"#
            ),
        ];
        test_encoding(tests)
    }

    #[test]
    fn pullreq_edits() {
        let tests = vec![(PullEditOptions::builder().title("test").build(),
                          r#"{"title":"test"}"#),
                         (PullEditOptions::builder().title("test").body("desc").build(),
                          r#"{"title":"test","body":"desc"}"#),
                         (PullEditOptions::builder().state("closed").build(),
                          r#"{"state":"closed"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn status_reqs() {
        let tests = vec![(StatusOptions::builder(StatusState::Pending).build(),
                  r#"{"state":"pending"}"#),
                 (StatusOptions::builder(StatusState::Success).target_url("http://acme.com").build(),
                  r#"{"state":"success","target_url":"http://acme.com"}"#),
                 (StatusOptions::builder(StatusState::Error).description("desc").build(),
                  r#"{"state":"error","description":"desc"}"#),
                 (StatusOptions::builder(StatusState::Failure)
                      .target_url("http://acme.com")
                      .description("desc")
                      .build(),
                  r#"{"state":"failure","target_url":"http://acme.com","description":"desc"}"#)];
        test_encoding(tests)
    }

    #[test]
    fn list_reqs() {
        fn test_serialize(tests: Vec<(IssueListOptions, &str)>) {
            for test in tests {
                match test {
                    (k, v) => assert_eq!(k.serialize(), v),
                }
            }
        }
        let tests = vec![
            (
                IssueListOptions::builder().build(),
                "state=open&sort=created&direction=asc"
            ),
            (
                IssueListOptions::builder().state(StdState::Closed).build(),
                "state=closed&sort=created&direction=asc"
             ),
            (
                IssueListOptions::builder().labels(vec!["foo", "bar"]).build(),
                "state=open&sort=created&direction=asc&labels=foo%2Cbar"
            ),
        ];
        test_serialize(tests)
    }
}
