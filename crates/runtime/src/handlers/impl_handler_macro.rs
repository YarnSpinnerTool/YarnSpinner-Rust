macro_rules! impl_handler {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: Fn($($param:ty)?) $(+ $clone:ident)?);) => {
        impl_handler_inner! {
            $(#[$attr])*
            pub struct $struct_name(pub $trait_name: Fn($($param)?)$(+ $clone)?),
        }
    };
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: FnMut($($param:ty)?) $(+ $clone:ident)?);) => {
        impl_handler_inner! {
            $(#[$attr])*
            pub struct $struct_name(pub $trait_name: FnMut($($param)?)$(+ $clone)?), mut
        }
    };
    ($($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: $fun:ident($($param:ty)?) $(+ $clone:ident)?);)+) => {
        $(
            impl_handler! {
                $(#[$attr])*
                pub struct $struct_name(pub $trait_name: $fun($($param)?)$(+ $clone)?);
            }
        )+
    };
}

macro_rules! impl_handler_inner {
    ($(#[$attr:meta])* pub struct $struct_name:ident(pub $trait_name:ident: $fun:ident($($param:ty)?) $(+ $clone:ident)?), $($mutable:ident)?) => {
        $(#[$attr])*
        pub type $struct_name = Box<dyn $trait_name + Send + Sync>;

        $(
            impl $clone for $struct_name {
                fn clone(&self) -> Self {
                    self.clone_box()
                }
            }
        )?

        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($struct_name))
            }
        }

        pub trait $trait_name: Send + Sync {
            fn call(&$($mutable)? self, $(param: $param,)? dialogue: &$($mutable)? HandlerSafeDialogue);
            $(
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn clone_box(&self) -> $struct_name {
                    let $clone = ();
                    unimplemented!()
                }
            )?
        }

        impl<T> $trait_name for T
        where
            T: $fun($($param,)? &$($mutable)? HandlerSafeDialogue) $(+ $clone)? + Send + Sync + 'static,
        {
            fn call(&$($mutable)? self, $(param: $param,)? dialogue: &$($mutable)? HandlerSafeDialogue){
                self($(param as $param,)? dialogue)
            }

            $(
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn clone_box(&self) -> $struct_name {
                    let $clone = ();
                    Box::new(self.clone())
                }
            )?
        }
    };
}
