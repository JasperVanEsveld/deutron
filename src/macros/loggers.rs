#[macro_export]
macro_rules! unwrap_log {
    (  $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => {
                BackendMessage::Info(InfoMessage::Error(err.to_string()))
                    .send()
                    .unwrap();
                return;
            }
        }
    };
    (  $e:expr, $source: expr, $proxy: expr ) => {
        match $source {
            Target::Window(source_id) => match $e {
                Ok(x) => x,
                Err(err) => {
                    let message = WindowMessage::Info(InfoMessage::Error(err.to_string()));
                    let log_res =
                        WebViewAction::Message($source, source_id, message).perform(&$proxy);
                    unwrap_log!(log_res);
                    return;
                }
            },
            Target::Backend => {
                unwrap_log!($e)
            }
        }
    };
}
