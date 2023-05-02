use crate::prelude::generated::yarnspinnerparser::{
    DialogueContextAttrs, HeaderContextAll, NodeContextAll, NodeContextAttrs,
};
use crate::prelude::*;
use antlr_rust::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use yarn_slinger_core::prelude::*;

pub(crate) fn validate_unique_node_names(
    mut state: CompilationIntermediate,
) -> CompilationIntermediate {
    // Ensure that all nodes names in this compilation are unique. Node
    // name uniqueness is important for several processes, so we do this
    // check here.
    let all_nodes = state
        .parsed_files
        .iter()
        .map(|file| file.tree.node_all().iter().map(|node| (node, file)))
        .flatten();

    // Pair up every node with its name, and filter out any that don't
    // have a name
    let nodes_with_names = all_nodes.filter_map(|(node, file)| {
        node.header_all()
            .iter()
            .find(|header| header.header_key.unwrap().get_text() == "title")
            .map(|title_header| {
                (
                    title_header.header_value.unwrap().get_text().to_owned(),
                    title_header.clone(),
                    node.clone(),
                    file,
                )
            })
    });

    let nodes_by_name = nodes_with_names.fold(
        HashMap::<String, Vec<(Rc<HeaderContextAll>, Rc<NodeContextAll>, &FileParseResult)>>::new(),
        |mut map, (name, header, node, file)| {
            map.entry(name)
                .or_insert_with(Vec::new)
                .push((header, node, file));
            map
        },
    );
    state
    /*
    var allNodes = parseResults.SelectMany(r =>
            {
                var dialogue = r.Tree.Payload as YarnSpinnerParser.DialogueContext;
                if (dialogue == null)
                {
                    return Enumerable.Empty<(YarnSpinnerParser.NodeContext Node, FileParseResult File)>();
                }

                return dialogue.node().Select(n => (Node: n, File: r));
            });

            // Pair up every node with its name, and filter out any that don't
            // have a name
            var nodesWithNames = allNodes.Select(n =>
            {
                var titleHeader = GetHeadersWithKey(n.Node, "title").FirstOrDefault();
                if (titleHeader == null)
                {
                    return (
                        Name: null,
                        Header: null,
                        Node: n.Node,
                        File: n.File);
                }
                else
                {
                    return (
                        Name: titleHeader.header_value.Text,
                        Header: titleHeader,
                        Node: n.Node,
                        File: n.File);
                }
            }).Where(kv => kv.Name != null);

            var nodesByName = nodesWithNames.GroupBy(n => n.Name);

            // Find groups of nodes with the same name and generate diagnostics
            // for each
            foreach (var group in nodesByName)
            {
                if (group.Count() == 1)
                {
                    continue;
                }

                // More than one node has this name! Report an error on both.
                foreach (var entry in group)
                {
                    var d = new Diagnostic(entry.File.Name, entry.Header, $"More than one node is named {entry.Name}");
                    diagnostics.Add(d);
                }
            }
     */
}
