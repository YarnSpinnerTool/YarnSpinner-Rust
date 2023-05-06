macro_rules! impl_handler {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: Fn($($param:ty)?));) => {
        impl_handler_inner! {
            $(#[$attr])*
            pub struct $struct_name(pub $trait_name: Fn($($param)?)),
        }
    };
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: FnMut($($param:ty)?));) => {
        impl_handler_inner! {
            $(#[$attr])*
            pub struct $struct_name(pub $trait_name: FnMut($($param)?)), mut
        }
    };
    ($($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: $fun:ident($($param:ty)?));)+) => {
        $(
            impl_handler! {
                $(#[$attr])*
                pub struct $struct_name(pub $trait_name: $fun($($param)?));
            }
        )+
    };
}

macro_rules! impl_handler_inner {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: $fun:ident($($param:ty)?)), $($mutable:ident)?) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $struct_name(pub Box<dyn $trait_name + Send + Sync>);

        impl std::ops::Deref for $struct_name {
            type Target = Box<dyn $trait_name + Send + Sync>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> From<T> for $struct_name
            where T: $fun($($param)?) + Clone + Send + Sync + 'static,
        {
            fn from(f: T) -> Self {
                Self(Box::new(f))
            }
        }

        impl Clone for Box<dyn $trait_name + Send + Sync> {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }

        impl std::fmt::Debug for dyn $trait_name + Send + Sync {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub trait $trait_name: Send + Sync {
            fn call(&$($mutable)? self, $(param: $param)?);
            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync>;
        }

        impl<T> $trait_name for T
        where
            T: $fun($($param)?) + Clone + Send + Sync + 'static,
        {
            fn call(&$($mutable)? self, $(param: $param)?) {
                self($(param as $param)?)
            }

            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync> {
                Box::new(self.clone())
            }
        }
    };
}
