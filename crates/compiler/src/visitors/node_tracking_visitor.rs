use crate::parser::generated::yarnspinnerparser::{self, *};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use std::collections::HashSet;

#[derive(Clone)]
pub(crate) struct NodeTrackingVisitor {
    tracking_nodes: HashSet<String>,
    node_deny_list: HashSet<String>,
    _dummy: Option<String>,
}

impl NodeTrackingVisitor {
    pub(crate) fn new(
        existing_tracked_nodes: HashSet<String>,
        existing_blocked_nodes: HashSet<String>,
    ) -> Self {
        Self {
            tracking_nodes: existing_tracked_nodes,
            node_deny_list: existing_blocked_nodes,
            _dummy: Default::default(),
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for NodeTrackingVisitor {
    type Node = YarnSpinnerParserContextType;
    type Return = Option<String>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for NodeTrackingVisitor {
    fn visit_node(&mut self, ctx: &NodeContext<'input>) -> Self::Return {
        let mut title = None;
        let mut tracking = None;
        for header in ctx.header_all() {
            let key = header.header_key.as_ref().unwrap().get_text();
            let value = header
                .header_value
                .as_ref()
                .map(|val| val.get_text().to_owned());
            match key {
                "title" => {
                    title = value;
                }
                "tracking" => {
                    tracking = value;
                }
                _ => {}
            }
        }
        if let Some(title) = title {
            if let Some(tracking) = tracking {
                match tracking.as_str() {
                    "always" => {
                        self.tracking_nodes.insert(title);
                    }
                    "never" => {
                        self.node_deny_list.insert(title);
                    }
                    _ => {}
                }
            }
        }
        if let Some(body) = ctx.body() {
            return self.visit(&*body);
        }
        None
    }

    fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>) -> Self::Return {
        ctx.get_token(yarnspinnerparser::STRING, 0)
            .unwrap()
            .get_text()
            .trim_matches('"')
            .to_owned()
            .into()
    }

    fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
        let function_name = ctx
            .get_token(yarnspinnerparser::FUNC_ID, 0)
            .unwrap()
            .get_text();

        if !["visited", "visited_count"].contains(&function_name.as_str()) {
            return None;
        }
        // we aren't bothering to test anything about the value itself
        // if it isn't a static string we'll get back null so can ignore it
        // if the func has more than one parameter later on it will cause an error so again can ignore
        let expression = ctx.expression(0).unwrap();
        let result = self.visit(&*expression);
        if let Some(result) = result {
            self.tracking_nodes.insert(result);
        }
        None
    }
}
