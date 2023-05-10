//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/DialogueTests.cs>
//!
//! ## Implementation notes
//! `TestDumpingCode` was not ported because `GetByteCode` is not used by a user directly and thus was not implemented at all.

use std::sync::{Arc, RwLock};
use test_base::prelude::*;
use yarn_slinger::prelude::*;

mod test_base;

#[test]
fn test_node_exists() {
    let path = space_demo_scripts_path().join("Sally.yarn");
    let test_base = TestBase::new();

    let result = Compiler::new()
        .read_file(path)
        .replace_library(test_base.library().clone())
        .compile()
        .unwrap();

    let mut dialogue = test_base.dialogue;
    dialogue.replace_program(result.program.unwrap());
    assert!(dialogue.node_exists("Sally"));

    // Test clearing everything
    dialogue.unload_all();
    assert!(!dialogue.node_exists("Sally"));
}

#[test]
#[ignore = "Requires analyzer to be implemented, see https://github.com/yarn-slinger/yarn-slinger/issues/85"]
fn test_analysis() {
    todo!("Not ported yet")
}

#[test]
#[should_panic(expected = "No node named \"THIS NODE DOES NOT EXIST\" has been loaded.")]
fn test_missing_node() {
    let path = test_data_path().join("TestCases").join("Smileys.yarn");

    let result = Compiler::new().read_file(path).compile().unwrap();

    let mut test_base = TestBase::new()
        .with_program(result.program.unwrap())
        .with_runtime_errors_do_not_cause_failure();
    test_base.dialogue.set_node("THIS NODE DOES NOT EXIST");
}

#[test]
fn test_getting_current_node_name() {
    let path = space_demo_scripts_path().join("Sally.yarn");
    let test_base = TestBase::new();

    let compiler = Compiler::new()
        .read_file(path)
        .replace_library(test_base.library().clone());
    let result = compile(compiler).unwrap_pretty();

    let mut dialogue = test_base.dialogue;
    dialogue.replace_program(result.program.unwrap());

    // dialogue should not be running yet
    assert!(dialogue.current_node().is_none());

    dialogue.set_node("Sally");
    assert_eq!(dialogue.current_node(), Some("Sally".to_string()));

    dialogue.stop();
    // Current node should now be null
    assert!(dialogue.current_node().is_none());
}

#[test]
fn test_getting_raw_source() {
    let path = test_data_path().join("Example.yarn");
    let mut test_base = TestBase::new();

    let compiler = Compiler::new().read_file(path).unwrap();
    let result = compile(compiler).unwrap_pretty();

    test_base = test_base.with_compilation(result);
    let dialogue = &test_base.dialogue;

    let source_id = dialogue.get_string_id_for_node("LearnMore").unwrap();
    let source = test_base
        .string_table()
        .get(&LineId(source_id))
        .unwrap()
        .text
        .clone();

    assert_eq!(source, "A: HAHAHA\n");
}

#[test]
fn test_getting_tags() {
    let path = test_data_path().join("Example.yarn");
    let mut test_base = TestBase::new();

    let compiler = Compiler::new().read_file(path).unwrap();
    let result = compile(compiler).unwrap_pretty();

    test_base = test_base.with_program(result.program.unwrap());
    let dialogue = &test_base.dialogue;

    let tags = dialogue.get_tags_for_node("LearnMore").unwrap();

    assert_eq!(tags, vec!["rawText"]);
}

#[test]
fn test_prepare_for_line() {
    let path = test_data_path().join("TaggedLines.yarn");

    let compiler = Compiler::new().read_file(path).unwrap();
    let result = compile(compiler).unwrap_pretty();

    let mut dialogue = TestBase::new().with_compilation(result).dialogue;

    let prepare_for_lines_was_called = Arc::new(RwLock::new(false));
    let prepare_for_lines_was_called_clone = prepare_for_lines_was_called.clone();
    dialogue.set_prepare_for_lines_handler(move |lines, _| {
        // When the Dialogue realises it's about to run the Start
        // node, it will tell us that it's about to run these two
        // line IDs
        assert_eq!(lines.len(), 2);
        assert!(lines.contains(&"line:test1".into()));
        assert!(lines.contains(&"line:test2".into()));

        // Ensure that these asserts were actually called
        *prepare_for_lines_was_called_clone.write().unwrap() = true;
    });

    dialogue.set_node_to_start();

    assert!(*prepare_for_lines_was_called.read().unwrap());
}

#[test]
fn test_function_argument_type_inference() {
    let mut test_base = TestBase::new();
    // Register some functions
    test_base
        .dialogue
        .library_mut()
        .register_function("ConcatString", |a: String, b: String| a + &b)
        .register_function("AddInt", |a: i32, b: i32| a + b)
        .register_function("AddFloat", |a: f32, b: f32| a + b)
        .register_function("NegateBool", |a: bool| !a);

    // Run some code to exercise these functions
    let source = "\
    <<declare $str = \"\">>
    <<declare $int = 0>>
    <<declare $float = 0.0>>
    <<declare $bool = false>>

    <<set $str = ConcatString(\"a\", \"b\")>>
    <<set $int = AddInt(1,2)>>
    <<set $float = AddFloat(1,2)>>
    <<set $bool = NegateBool(true)>>
    ";

    let compiler =
        Compiler::from_test_source(source).with_library(test_base.dialogue.library().clone());
    let result = compile(compiler).unwrap_pretty();

    let storage = test_base
        .with_compilation(result)
        .run_standard_testcase()
        .dialogue
        .variable_storage();

    // The values should be of the right type and value
    let str_value: String = storage.get("$str").unwrap().into();
    assert_eq!("ab", &str_value);

    let int_value: i32 = storage.get("$int").unwrap().try_into().unwrap();
    assert_eq!(3, int_value);

    let float_value: f32 = storage.get("$float").unwrap().try_into().unwrap();
    assert_eq!(3.0, float_value);

    let bool_value: bool = storage.get("$bool").unwrap().try_into().unwrap();
    assert!(!bool_value);
}

#[test]
fn test_selecting_option_from_inside_option_callback() {
    let mut test_base = TestBase::new().with_test_plan(
        TestPlan::new()
            .expect_option("option 1")
            .expect_option("option 2")
            .then_select(0)
            .expect_line("final line"),
    );
    let string_table = test_base.string_table_shared();
    let test_plan = test_base.test_plan_shared();
    test_base.dialogue.set_line_handler(move |line, dialogue| {
        let line_text = string_table
            .read()
            .unwrap()
            .get(&line.id)
            .unwrap()
            .text
            .clone();
        let parsed_text = dialogue.parse_markup(&line_text);
        test_plan.write().unwrap().as_mut().unwrap().next();
        let test_plan = test_plan.read().unwrap();
        let test_plan = test_plan.as_ref().unwrap();

        let expected_step = test_plan.next_expected_step;
        let expected_value = test_plan.next_step_value.clone().unwrap();
        assert_eq!(ExpectedStepType::Line, expected_step);
        assert_eq!(StepValue::String(parsed_text), expected_value);
    });

    let test_plan = test_base.test_plan_shared();
    let string_table = test_base.string_table_shared();
    test_base
        .dialogue
        .set_options_handler(move |options, dialogue| {
            test_plan.write().unwrap().as_mut().unwrap().next();
            // Assert that the list of options we were given is
            // identical to the list of options we expect
            let actual_options: Vec<_> = options
                .into_iter()
                .map(|o| {
                    let line =
                        get_composed_text_for_line_with_no_self(&o.line, &string_table, dialogue);
                    ProcessedOption {
                        line,
                        enabled: o.is_available,
                    }
                })
                .collect();
            let test_plan = test_plan.read().unwrap();
            let test_plan = test_plan.as_ref().unwrap();
            let next_expected_options = test_plan.next_expected_options.clone();
            assert_eq!(next_expected_options, actual_options);

            let expected_step = test_plan.next_expected_step;
            assert_eq!(ExpectedStepType::Select, expected_step);
            dialogue.set_selected_option(OptionId::construct_for_debugging(0));
        });

    let test_plan = test_base.test_plan_shared();
    test_base
        .dialogue
        .set_command_handler(move |_command, _dialogue| {
            test_plan.write().unwrap().as_mut().unwrap().next();
            let expected_step = test_plan
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .next_expected_step;
            assert_eq!(ExpectedStepType::Command, expected_step);
        });

    let test_plan = test_base.test_plan_shared();
    test_base
        .dialogue
        .set_dialogue_complete_handler(move |_dialogue| {
            test_plan.write().unwrap().as_mut().unwrap().next();
            let expected_step = test_plan
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .next_expected_step;
            assert_eq!(ExpectedStepType::Stop, expected_step);
        });

    let compiler = Compiler::from_test_source("-> option 1\n->option 2\nfinal line\n");
    let result = compile(compiler).unwrap_pretty();

    test_base.with_compilation(result).run_standard_testcase();
}
