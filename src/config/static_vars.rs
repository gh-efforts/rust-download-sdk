use super::{build_http_client, configurable::Configurable, load_config};
use once_cell::sync::OnceCell;
use reqwest::blocking::Client as HTTPClient;
use std::sync::RwLock;

#[cfg(not(test))]
mod safe {
    use super::*;

    pub(in super::super) static QINIU_CONFIG: OnceCell<RwLock<Option<Configurable>>> =
        OnceCell::new();

    #[inline]
    pub(in super::super) fn qiniu_config() -> &'static RwLock<Option<Configurable>> {
        QINIU_CONFIG.get_or_init(|| RwLock::new(load_config()))
    }

    static HTTP_CLIENT: OnceCell<RwLock<HTTPClient>> = OnceCell::new();

    #[inline]
    pub(crate) fn http_client() -> &'static RwLock<HTTPClient> {
        HTTP_CLIENT.get_or_init(build_http_client)
    }

    type ConfigUpdateHandlers = Vec<fn()>;
    static CONFIG_UPDATE_HANDLERS: OnceCell<RwLock<ConfigUpdateHandlers>> = OnceCell::new();

    pub(in super::super) fn config_update_handlers() -> &'static RwLock<ConfigUpdateHandlers> {
        CONFIG_UPDATE_HANDLERS.get_or_init(Default::default)
    }
}

#[cfg(not(test))]
pub(crate) use safe::*;

#[cfg(test)]
mod not_safe {
    use super::{super::watcher::unwatch_all, *};

    pub(in super::super) static mut QINIU_CONFIG: OnceCell<RwLock<Option<Configurable>>> =
        OnceCell::new();

    #[inline]
    pub(in super::super) fn qiniu_config() -> &'static RwLock<Option<Configurable>> {
        unsafe { &mut QINIU_CONFIG }.get_or_init(|| RwLock::new(load_config()))
    }

    static mut HTTP_CLIENT: OnceCell<RwLock<HTTPClient>> = OnceCell::new();

    #[inline]
    pub(crate) fn http_client() -> &'static RwLock<HTTPClient> {
        unsafe { &mut HTTP_CLIENT }.get_or_init(build_http_client)
    }

    type ConfigUpdateHandlers = Vec<fn()>;
    static mut CONFIG_UPDATE_HANDLERS: OnceCell<RwLock<ConfigUpdateHandlers>> = OnceCell::new();

    pub(in super::super) fn config_update_handlers() -> &'static RwLock<ConfigUpdateHandlers> {
        unsafe { &mut CONFIG_UPDATE_HANDLERS }.get_or_init(Default::default)
    }

    pub(in super::super) fn reset_static_vars() {
        unsafe { &mut QINIU_CONFIG }.take();
        unsafe { &mut HTTP_CLIENT }.take();
        unsafe { &mut CONFIG_UPDATE_HANDLERS }.take();
        unwatch_all().unwrap();
    }
}
#[cfg(test)]
pub(crate) use not_safe::*;
