use std::rc::Rc;
use crate::packagemanager::WhackPackage;

/// Directed acyclic graph of the dependency tree.
pub struct Dag {
    pub edges: Vec<DagEdge>,
}

impl Dag {
    /// Retrieves the directed acyclic graph of the dependency tree.
    pub fn retrieve(package: Option<String>) -> Result<Dag, DagError> {
        // Read the Whack manifest
        fixme();

        // If current project is a workspace, then require a package name
        // to be specified, at which the check process executes.
        fixme();

        // Check for manifest updates.
        fixme();

        // If the manifest has been updated, update dependencies
        // and clear up the build script's artifacts.
        // Remember that the lock file must be considered for the
        // exact versions of registry dependencies.
        fixme();

        // Build a directed acyclic graph (DAG) of the dependencies.
        fixme();
    }
}

pub struct DagEdge {
    pub from: Rc<WhackPackage>,
    pub to: Rc<WhackPackage>,
}

pub enum DagError {
    ManifestNotFound,
    PackageMustBeSpecified,
}