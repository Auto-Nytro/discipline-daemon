use serde::{Serialize, Deserialize};
use crate::{App, IsOperation, OperatingSystemUsername};
use crate::networking_access::database_procedures;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Error {
  NoSuchEnforcer,
  EnforcerAlreadyEnabled,
  InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
  username: OperatingSystemUsername
}

impl IsOperation for Operation {
  type Outcome = Result<(), Error>;

  fn execute(self, app: &mut App) -> Self::Outcome {
    let feature = &mut app.app_data.networking_access;

    let Some(enforcer) = feature
      .enforcers
      .iter_mut()
      .find(|enforcer| enforcer.username == self.username) else 
    {
      return Err(Error::NoSuchEnforcer);
    };

    if enforcer.is_enabled {
      return Err(Error::EnforcerAlreadyEnabled);
    }

    if let Err(_) = database_procedures::enable_enforcer(
      &app.database_connection, 
      &self.username,
    ) {
      return Err(Error::InternalError);
    }

    enforcer.is_enabled = true;
    Ok(())
  }
}

