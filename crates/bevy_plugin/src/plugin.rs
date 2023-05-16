use bevy::prelude::*;
use yarn_slinger::prelude::*;

pub struct YarnSlingerPlugin;

impl Plugin for YarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Compiler>()
            .register_type::<File>()
            .register_type::<CompilationType>()
            .register_type::<Compilation>()
            .register_type::<CompilerError>()
            .register_type::<Diagnostic>()
            .register_type::<DiagnosticSeverity>()
            .register_type::<DebugInfo>()
            .register_type::<LineInfo>()
            .register_type::<Declaration>()
            .register_type::<DeclarationSource>()
            .register_type::<StringInfo>()
            .register_type::<LineId>()
            .register_type::<Position>()
            .register_type::<YarnValue>()
            .register_type::<InvalidOpCodeError>()
            .register_type::<Program>()
            .register_type::<yarn_slinger::prelude::Node>()
            .register_type::<Header>()
            .register_type::<Instruction>()
            .register_type::<Type>()
            .register_type::<Command>()
            .register_type::<Dialogue>()
            .register_type::<DialogueOption>()
            .register_type::<OptionId>()
            .register_type::<DialogueEvent>()
            .register_type::<Line>()
            .register_type::<Diagnosis>()
            .register_type::<DiagnosisSeverity>()
            .register_type::<MarkupParseError>()
            .register_type::<MarkupAttribute>()
            .register_type::<MarkupValue>();
    }
}
