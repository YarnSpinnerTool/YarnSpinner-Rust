use crate::prelude::*;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::*;
use bevy::utils::{all_tuples, HashMap};
use std::fmt::Debug;

pub(crate) fn commands_plugin(_app: &mut App) {}

pub trait YarnCommand<Marker>: Send + Sync + 'static {
    type In: YarnCommandIn;
    type Param: SystemParam;

    fn run(&mut self, input: Self::In, param_value: <Self::Param as SystemParam>::Item<'_, '_>);
}

pub trait YarnCommandIn {
    type Item: YarnCommandIn;
    fn from_params(name: &str, params: &mut Vec<YarnValue>) -> Self::Item
    where
        Self: Sized;
}

/// Adapted from <https://github.com/bevyengine/bevy/blob/fe852fd0adbce6856f5886d66d20d62cfc936287/crates/bevy_ecs/src/system/system_param.rs#L1370>
macro_rules! impl_command_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        #[allow(unused_variables)]`
        impl<$($param,)*> YarnCommandIn for ($($param,)*)
        where
            $($param: YarnCommandIn,)*
        {
            type Item = ($($param::Item,)*);


            #[allow(non_snake_case)]
            fn from_params(name: &str, params: &mut Vec<YarnValue>) -> Self::Item
            where
                Self: Sized,
            {
                ($($param::from_params(name, params),)*)
            }
        }
    };
}

all_tuples!(impl_command_tuple, 0, 16, P);

impl YarnCommandIn for usize {
    type Item = usize;

    fn from_params(name: &str, params: &mut Vec<YarnValue>) -> Self::Item
    where
        Self: Sized,
    {
        params
            .pop()
            .unwrap_or_else(|| panic!("Yarn command {name} was called with too few params"))
            .try_into()
            .unwrap_or_else(|e| panic!("Passed an invalid param to Yarn command {name}: {e}"))
    }
}
