#[cfg(feature = "use_newrelic")]
#[macro_use]
pub mod detail {
    use lazy_static::lazy_static;
    use newrelic::{App, Transaction};
    use std::env;

    const APP_NAME: &str = "ishocon2-rust-hiratasa";

    lazy_static! {
        pub static ref APP: NewRelicAppData = NewRelicAppData::new();
    }

    // For handy access to global instance outside this module.
    #[allow(unused_macros)]
    macro_rules! newrelic_app {
        () => {
            crate::newrelic_util::detail::APP
        };
    }

    #[allow(unused_macros)]
    macro_rules! newrelic_init {
        () => {
            lazy_static::initialize(&newrelic_app!());
        };
    }

    #[allow(unused_macros)]
    macro_rules! newrelic_transaction {
        ($name:expr) => {
            newrelic_app!().transaction($name)
        };
    }

    #[allow(unused_macros)]
    macro_rules! nrdb {
        ($tr:expr,$f:expr) => {
            if let Some(tr) = $tr.as_ref() {
                tr.datastore_segment(
                    &newrelic::DatastoreParamsBuilder::new(newrelic::Datastore::MySQL)
                        .operation(stringify!($f))
                        .build()
                        .expect("Invalid datastore segment parameters"),
                    |_| $f,
                )
            } else {
                $f
            }
        };
    }

    pub struct NewRelicAppData {
        app: Option<App>,
    }

    impl NewRelicAppData {
        pub fn new() -> NewRelicAppData {
            match env::var("NEW_RELIC_LICENSE_KEY") {
                Ok(key) => {
                    let app = App::new(APP_NAME, &key).expect("Could not create app.");
                    NewRelicAppData { app: Some(app) }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    NewRelicAppData { app: None }
                }
            }
        }

        pub fn transaction(&self, name: &str) -> Option<Transaction> {
            self.app.as_ref().map(|app| {
                app.web_transaction(name)
                    .expect("Could not start transaction")
            })
        }
    }

    mod actix_web {
        use actix_service::*;
        use actix_web::dev::*;
        use actix_web::Error;
        use core::future::Future;
        use futures::FutureExt;

        use super::NewRelicAppData;

        // Use with App::wrap_fn
        #[allow(dead_code)]
        fn wrap_log_transaction<S, Res>(
            req: ServiceRequest,
            srv: &mut S,
        ) -> impl Future<Output = Result<Res, Error>>
        where
            S: Service<Request = ServiceRequest, Response = Res, Error = Error>,
        {
            let newrelic: &NewRelicAppData = &req.app_data().unwrap();

            let transaction = newrelic.transaction(&(req.method().to_string() + req.path()));

            srv.call(req).map(|res| {
                // take ownership
                let _transaction = transaction;
                res
            })
        }
    }
}

#[cfg(not(feature = "use_newrelic"))]
#[macro_use]
mod detail {
    #[allow(unused_macros)]
    macro_rules! newrelic_init {
        () => {};
    }

    #[allow(unused_macros)]
    macro_rules! newrelic_transaction {
        ($($_:expr),*) => {
            ()
        };
    }

    #[allow(unused_macros)]
    macro_rules! nrdb {
        ($tr:expr,$f:expr) => {
            $f
        };
    }
}
