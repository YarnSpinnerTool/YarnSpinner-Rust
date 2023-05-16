use bevy::prelude::*;
use yarn_slinger::prelude::*;

pub struct YarnSlingerPlugin;

impl Plugin for YarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<compiler::Compiler>()
            .register_type::<compiler::File>()
            .register_type::<compiler::CompilationType>()
            .register_type::<compiler::Compilation>()
            .register_type::<compiler::CompilationError>()
            .register_type::<compiler::Diagnostic>()
            .register_type::<compiler::DiagnosticSeverity>()
            .register_type::<compiler::DebugInfo>()
            .register_type::<compiler::LineInfo>()
            .register_type::<compiler::Declaration>()
            .register_type::<compiler::DeclarationSource>()
            .register_type::<compiler::StringInfo>()
            .register_type::<LineId>()
            .register_type::<Position>()
            .register_type::<YarnValue>()
            .register_type::<InvalidOpCodeError>()
            .register_type::<Program>()
            .register_type::<yarn_slinger::prelude::Node>()
            .register_type::<Header>()
            .register_type::<Instruction>()
            .register_type::<instruction::OpCode>()
            .register_type::<Operand>()
            .register_type::<Type>()
            .register_type::<operand::Value>()
            .register_type::<runtime::Command>()
            .register_type::<runtime::Dialogue>()
            .register_type::<runtime::DialogueOption>()
            .register_type::<runtime::OptionId>()
            .register_type::<runtime::DialogueEvent>()
            .register_type::<runtime::Line>()
            .register_type::<runtime::Diagnosis>()
            .register_type::<runtime::DiagnosisSeverity>()
            .register_type::<runtime::MarkupParseError>();
    }
}
