//! An enum defining the possible actions which can be sent to Amazon ECS.

use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Copy, Debug)]
pub enum ECSAction {
    ListClusters,
}

/// Used primarily to map an ECSAction to its corresponding string representation to be used in
/// the X-Amz-Target header.
impl Display for ECSAction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let action_str = match self {
            &ECSAction::ListClusters => "ListClusters",
        };

        write!(f, "{}", action_str)
    }
}
