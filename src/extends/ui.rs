mod backend;
mod backend_type;

use crate::ui;
use anyhow::Result;

pub struct Return<T> {
    error: ui::Error,
    value: T,
}

impl<T> Return<T> {
    pub fn into_error(self) -> ui::Error {
        self.error
    }

    pub fn into_tuple<U>(self) -> (ui::Error, U)
    where
        U: From<T>,
    {
        (self.error, self.value.into())
    }
}

impl From<anyhow::Error> for ui::Error {
    fn from(value: anyhow::Error) -> Self {
        ui::Error {
            did_occured: true,
            message: value.to_string().into(),
        }
    }
}

impl<T> From<Result<T>> for Return<T>
where
    T: Default,
{
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(ok) => Return {
                error: ui::Error {
                    did_occured: false,
                    message: "".into(),
                },
                value: ok,
            },
            Err(err) => Return {
                error: err.into(),
                value: T::default(),
            },
        }
    }
}
