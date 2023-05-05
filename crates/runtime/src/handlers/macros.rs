macro_rules! impl_function_newtype_with_no_params {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: FnMut())) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $struct_name(pub Box<dyn $trait_name + Send + Sync>);

        impl Clone for Box<dyn $trait_name + Send + Sync> {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }

        impl Debug for dyn $trait_name + Send + Sync {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub trait $trait_name {
            fn call(&mut self);
            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync>;
        }

        impl<T> $trait_name for T
        where
            T: FnMut() + Clone + Send + Sync + 'static,
        {
            fn call(&mut self) {
                self()
            }

            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync> {
                Box::new(self.clone())
            }
        }
    };
}

macro_rules! impl_function_newtype_mut {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: FnMut($param:ty))) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $struct_name(pub Box<dyn $trait_name + Send + Sync>);

        impl Deref for $struct_name {
            type Target = Box<dyn $trait_name + Send + Sync>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Clone for Box<dyn $trait_name + Send + Sync> {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }

        impl Debug for dyn $trait_name + Send + Sync {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub trait $trait_name: Send + Sync {
            fn call(&mut self, param: $param);
            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync>;
        }

        impl<T> $trait_name for T
        where
            T: FnMut($param) + Clone + Send + Sync + 'static,
        {
            fn call(&mut self, param: $param) {
                self(param)
            }

            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync> {
                Box::new(self.clone())
            }
        }
    };
}

macro_rules! impl_function_newtype {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: Fn($param:ty))) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $struct_name(pub Box<dyn $trait_name + Send + Sync>);

        impl Deref for $struct_name {
            type Target = Box<dyn $trait_name + Send + Sync>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Clone for Box<dyn $trait_name + Send + Sync> {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }

        impl Debug for dyn $trait_name + Send + Sync {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub trait $trait_name: Send + Sync {
            fn call(&self, param: $param);
            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync>;
        }

        impl<T> $trait_name for T
        where
            T: Fn($param) + Clone + Send + Sync + 'static,
        {
            fn call(&self, param: $param) {
                self(param)
            }

            fn clone_box(&self) -> Box<dyn $trait_name + Send + Sync> {
                Box::new(self.clone())
            }
        }
    };
}
