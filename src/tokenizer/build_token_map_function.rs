use crate::types::token::Token;
use crate::types::token::emojis;
use std::collections::HashMap;

pub fn build_token_map() -> HashMap<&'static str, Token> {
    let mut m = HashMap::new();
    // Prelude
    m.insert(emojis::true_token::EMOJI, emojis::true_token::to_token());
    m.insert(emojis::true_token::ASCII_EQUIVALENT, emojis::true_token::to_token());
    m.insert(emojis::false_token::EMOJI, emojis::false_token::to_token());
    m.insert(emojis::false_token::ASCII_EQUIVALENT, emojis::false_token::to_token());
    m.insert(emojis::func_start_token::EMOJI, emojis::func_start_token::to_token());
    m.insert(emojis::forall_token::EMOJI, emojis::forall_token::to_token());
    m.insert(emojis::exists_token::EMOJI, emojis::exists_token::to_token());
    m.insert(emojis::up_arrow_token::EMOJI, emojis::up_arrow_token::to_token());
    m.insert(emojis::and_token::EMOJI, emojis::and_token::to_token());
    m.insert(emojis::or_token::EMOJI, emojis::or_token::to_token());
    m.insert(emojis::not_token::EMOJI, emojis::not_token::to_token());
    m.insert(emojis::implies_token::EMOJI, emojis::implies_token::to_token());
    m.insert(emojis::iff_token::EMOJI, emojis::iff_token::to_token());
    m.insert(emojis::s_token::EMOJI, emojis::s_token::to_token());
    m.insert(emojis::k_token::EMOJI, emojis::k_token::to_token());
    m.insert(emojis::i_token::EMOJI, emojis::i_token::to_token());
    m.insert(emojis::sparkle_token::EMOJI, emojis::sparkle_token::to_token());
    m.insert(emojis::lightning_token::EMOJI, emojis::lightning_token::to_token());
    m.insert(emojis::b_token::EMOJI, emojis::b_token::to_token());
    m.insert(emojis::c_token::EMOJI, emojis::c_token::to_token());
    m.insert(emojis::w_token::EMOJI, emojis::w_token::to_token());
    m.insert(emojis::y_token::EMOJI, emojis::y_token::to_token());
    m.insert(emojis::z_token::EMOJI, emojis::z_token::to_token());
    m.insert(emojis::omega_token::EMOJI, emojis::omega_token::to_token());
    m.insert(emojis::lambda_token::EMOJI, emojis::lambda_token::to_token());
    m.insert(emojis::top_token::EMOJI, emojis::top_token::to_token());
    m.insert(emojis::bottom_token::EMOJI, emojis::bottom_token::to_token());
    m.insert(emojis::maps_to_token::EMOJI, emojis::maps_to_token::to_token());
    m.insert(emojis::compose_token::EMOJI, emojis::compose_token::to_token());
    m.insert(emojis::equals_token::EMOJI, emojis::equals_token::to_token());
    m.insert(emojis::not_equals_token::EMOJI, emojis::not_equals_token::to_token());
    m.insert(emojis::identical_token::EMOJI, emojis::identical_token::to_token());
    m.insert(emojis::proves_token::EMOJI, emojis::proves_token::to_token());
    m.insert(emojis::entails_token::EMOJI, emojis::entails_token::to_token());

    // WASM Compiler Prelude (conceptual)
    m.insert(emojis::compiler_token::EMOJI, emojis::compiler_token::to_token());
    m.insert(emojis::optimizer_token::EMOJI, emojis::optimizer_token::to_token());
    m.insert(emojis::box_token::EMOJI, emojis::box_token::to_token()); // Re-used for emit_wat_block
    m.insert(emojis::check_trap_token::EMOJI, emojis::check_trap_token::to_token());

    // Rules (emoji -> token mapping)
    m.insert(emojis::return_token::EMOJI, emojis::return_token::to_token());
    m.insert(emojis::call_token::EMOJI, emojis::call_token::to_token());
    m.insert(emojis::local_get_token::EMOJI, emojis::local_get_token::to_token());
    m.insert(emojis::local_set_token::EMOJI, emojis::local_set_token::to_token());
    m.insert(emojis::spawn_token_token::EMOJI, emojis::spawn_token_token::to_token());
    m.insert(emojis::emit_wat_block_token::EMOJI, emojis::emit_wat_block_token::to_token());
    m.insert(emojis::emit_wat_block_token::ASCII_EQUIVALENT, emojis::emit_wat_block_token::to_token());
    m.insert(emojis::rule_entry_token::EMOJI, emojis::rule_entry_token::to_token()); // This will need special handling
    m.insert(emojis::apply_rules_loop_token::EMOJI, emojis::apply_rules_loop_token::to_token());

    // World Tape
    m.insert(emojis::add_token::EMOJI, emojis::add_token::to_token());
    m.insert(emojis::sub_token::EMOJI, emojis::sub_token::to_token());
    m.insert(emojis::mul_token::EMOJI, emojis::mul_token::to_token());
    m.insert(emojis::div_s_token::EMOJI, emojis::div_s_token::to_token());
    m.insert(emojis::gt_s_token::EMOJI, emojis::gt_s_token::to_token());
    m.insert(emojis::zos_export_token::EMOJI, emojis::zos_export_token::to_token());
    m.insert(emojis::zos_ready_token::EMOJI, emojis::zos_ready_token::to_token());
    m.insert(emojis::newline_token::EMOJI, emojis::newline_token::to_token());
    m.insert(emojis::newline_token::ASCII_EQUIVALENT, emojis::newline_token::to_token());
    m.insert(emojis::i32_const_token::EMOJI, emojis::i32_const_token::to_token());
    m.insert(emojis::i32_const_token::ASCII_EQUIVALENT, emojis::i32_const_token::to_token());
    m.insert(emojis::f32_const_token::EMOJI, emojis::f32_const_token::to_token());
    m.insert(emojis::f32_const_token::ASCII_EQUIVALENT, emojis::f32_const_token::to_token());
    m.insert(emojis::drop_token::EMOJI, emojis::drop_token::to_token());

    m
}
