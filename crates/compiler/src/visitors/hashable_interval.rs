use crate::parser::generated::yarnspinnerparser::YarnSpinnerParserContext;
use crate::prelude::*;
use antlr_rust::interval_set::Interval;
use antlr_rust::parser_rule_context::ParserRuleContext;
use rusty_yarn_spinner_core::types::Type;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct HashableInterval(Interval);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct KnownTypes(pub(crate) HashMap<HashableInterval, Type>);

impl KnownTypes {
    pub(crate) fn get<'input>(&self, ctx: &impl YarnSpinnerParserContext<'input>) -> Option<&Type> {
        let hashable_interval = ctx.get_hashable_interval();
        self.0.get(&hashable_interval)
    }

    pub(crate) fn get_mut<'input>(
        &mut self,
        ctx: &impl YarnSpinnerParserContext<'input>,
    ) -> Option<&mut Type> {
        let hashable_interval = ctx.get_hashable_interval();
        self.0.get_mut(&hashable_interval)
    }

    pub(crate) fn insert<'input>(
        &mut self,
        ctx: &impl YarnSpinnerParserContext<'input>,
        r#type: impl Into<Option<Type>>,
    ) -> Option<Type> {
        let r#type = r#type.into()?;
        let hashable_interval = ctx.get_hashable_interval();
        self.0.insert(hashable_interval, r#type)
    }

    pub(crate) fn extend(&mut self, other: Self) {
        self.0.extend(other.0)
    }
}

impl From<Interval> for HashableInterval {
    fn from(interval: Interval) -> Self {
        Self(interval)
    }
}

impl Ord for HashableInterval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.a.cmp(&other.0.a).then(self.0.b.cmp(&other.0.b))
    }
}

impl PartialOrd for HashableInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for HashableInterval {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.a.hash(state);
        self.0.b.hash(state);
    }
}

impl Deref for HashableInterval {
    type Target = Interval;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HashableInterval {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for KnownTypes {
    type Target = HashMap<HashableInterval, Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KnownTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(crate) trait GetHashableInterval<'input>: ParserRuleContext<'input> {
    fn get_hashable_interval(&self) -> HashableInterval {
        let interval = self.get_source_interval();
        HashableInterval(interval)
    }
}

impl<'input, T: ParserRuleContext<'input>> GetHashableInterval<'input> for T {}
