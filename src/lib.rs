pub mod data {
    pub mod datasource {
        pub mod document {
            pub mod base;
        }
        pub mod file {
            pub mod base;
           
        }
        pub mod relational {
            pub mod base;
            pub mod mariadb;
           
        }
        pub mod base;
    }

    pub mod datasource_factory;
}

pub mod config {
    pub mod configuration;
    pub mod shared;
    pub mod specific {
        pub mod api_config;
        pub mod auth_config;
        pub mod cors_config;
        pub mod database_config;
        pub mod documentation_config;
        pub mod entity_config;
        pub mod server_config;
    }
}

pub mod api {
    pub mod datasource_integration;

    pub mod adapters {
        pub mod api_adapter;
    }

    pub mod common {
        pub mod api_entity;
    }

    pub mod rocket {
        pub mod rocket_adapter;
       
        pub mod handlers {
            pub mod catch_all;   
        }
    }

    pub mod handlers {
        pub mod manager;
        pub mod common {
            pub mod utils;
        }

        pub mod crud {
            pub mod create;
            pub mod delete;
            pub mod read;
            pub mod update;
            pub mod list;
        }

        pub mod custom {
            pub mod routes;
        }
    }
}
pub mod serialization {
    pub mod serialization_service;
}

pub mod connection {
    pub mod connection_manager;
}

pub mod error;
pub mod filemanager;
pub mod request_manager;

// Re-export commonly used items
pub use error::RusterApiError;
pub type Result<T> = std::result::Result<T, RusterApiError>;
