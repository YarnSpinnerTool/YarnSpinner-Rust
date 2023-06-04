use crate::example_ui::setup::{create_dialog_text, DialogueNode};
use crate::prelude::LocalizedLine;
use bevy::prelude::*;
use bevy::utils::Instant;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn typewriter_plugin(app: &mut App) {
    app.add_system(write_text.run_if(resource_exists::<Typewriter>()));
}

#[derive(Debug, Clone, PartialEq, Resource)]
pub(crate) struct Typewriter {
    pub(crate) character_name: Option<String>,
    pub(crate) current_text: String,
    pub(crate) graphemes_left: Vec<String>,
    pub(crate) last_before_options: bool,
    elapsed: f32,
    start: Instant,
    fast_typing: bool,
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            character_name: default(),
            current_text: default(),
            graphemes_left: default(),
            last_before_options: default(),
            elapsed: default(),
            start: Instant::now(),
            fast_typing: default(),
        }
    }
}

impl Typewriter {
    pub(crate) fn set_line(&mut self, line: &LocalizedLine) {
        *self = Self {
            character_name: line.character_name().map(|s| s.to_string()),
            current_text: String::new(),
            graphemes_left: line
                .text_without_character_name()
                .graphemes(true)
                .map(|s| s.to_string())
                .collect(),
            last_before_options: line.is_last_line_before_options(),
            elapsed: 0.0,
            start: Instant::now(),
            fast_typing: false,
        };
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty()
    }

    pub(crate) fn fast_forward(&mut self) {
        self.fast_typing = true;
    }

    fn update_current_text(&mut self) {
        if self.is_finished() {
            return;
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();
        let calculated_graphemes = (self.graphemes_per_second() * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = (calculated_graphemes).min(graphemes_left);
        self.elapsed -= grapheme_length_to_take as f32 / self.graphemes_per_second();
        let graphemes_to_take = self.graphemes_left.drain(..grapheme_length_to_take);
        self.current_text.extend(graphemes_to_take);
    }

    fn graphemes_per_second(&self) -> f32 {
        if self.fast_typing {
            110.0
        } else {
            30.0
        }
    }
}

fn write_text(mut text: Query<&mut Text, With<DialogueNode>>, mut typewriter: ResMut<Typewriter>) {
    let mut text = text.single_mut();
    typewriter.update_current_text();

    let name = typewriter.character_name.as_deref();
    let current_text = &typewriter.current_text;
    let rest = typewriter.graphemes_left.join("");
    *text = create_dialog_text(name, current_text, rest);
}
