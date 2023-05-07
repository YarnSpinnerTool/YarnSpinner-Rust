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
        pub type $struct_name = Box<dyn $trait_name + Send + Sync>;

        impl Clone for $struct_name {
            fn clone(&self) -> Self {
                self.clone_box()
            }
        }

        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub trait $trait_name: Send + Sync {
            fn call(&$($mutable)? self, $(param: $param,)? dialogue: &$($mutable)? HandlerSafeDialogue);
            fn clone_box(&self) -> $struct_name;
        }

        impl<T> $trait_name for T
        where
            T: $fun($($param,)? &$($mutable)? HandlerSafeDialogue) + Clone + Send + Sync + 'static,
        {
            fn call(&$($mutable)? self, $(param: $param,)? dialogue: &$($mutable)? HandlerSafeDialogue){
                self($(param as $param,)? dialogue)
            }

            fn clone_box(&self) -> $struct_name {
                Box::new(self.clone())
            }
        }
    };
}
