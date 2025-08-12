# Emojitape Interpreter Documentation Index

This document serves as the central index for the `emojitape_interpreter` project's documentation. It provides an overview of the project's structure and links to detailed "index cards" for individual files, directories, and key concepts.

## Project Overview

- [README.md](index_cards/README.md.md): High-level project description and usage.
- [Cargo.toml](index_cards/Cargo.toml.md): Project configuration and dependencies.

## Directory Structure

- [src/ Directory](index_cards/src_directory.md): Core Rust source code.
- [tapes/ Directory](index_cards/tapes_directory.md): Emojitape example programs and test cases.
- [output/ Directory](index_cards/output_directory.md): Generated and transformed output files.

### src/types/token/ Module

- [src/types/token/ Directory](index_cards/src_types_token_directory.md): Definitions and implementations for `Token` enum.
- [src/types/token/mod.rs](index_cards/src_types_token_mod.rs.md): Main module file for `Token`.
- [src/types/token/executable.rs](index_cards/src_types_token_executable.rs.md): Defines the `ExecutableToken` trait.
- [src/types/token/executable_impl.rs](index_cards/src_types_token_executable_impl.rs.md): Implements `ExecutableToken` for `Token`.
- [src/types/token/impl_token_display_fromstr.rs](index_cards/src_types_token_impl_token_display_fromstr.rs.md): `Display` and `FromStr` implementations for `Token`.
- [src/types/token/tests.rs](index_cards/src_types_token_tests.rs.md): Unit tests for token functionalities.

#### src/types/token/impl_token/ Execution Logic

- [add.rs](index_cards/src_types_token_impl_token_add.rs.md): `Add` token execution logic.
- [and.rs](index_cards/src_types_token_impl_token_and.rs.md): `And` token execution logic.
- [box.rs](index_cards/src_types_token_impl_token_box.rs.md): `Box` token execution logic.
- [call.rs](index_cards/src_types_token_impl_token_call.rs.md): `Call` token execution logic.
- [comment.rs](index_cards/src_types_token_impl_token_comment.rs.md): `Comment` token execution logic.
- [div_s.rs](index_cards/src_types_token_impl_token_div_s.rs.md): `DivS` token execution logic.
- [drop.rs](index_cards/src_types_token_impl_token_drop.rs.md): `Drop` token execution logic.
- [emit_wat_block.rs](index_cards/src_types_token_impl_token_emit_wat_block.rs.md): `EmitWatBlock` token execution logic.
- [equals.rs](index_cards/src_types_token_impl_token_equals.rs.md): `Equals` token execution logic.
- [gt_s.rs](index_cards/src_types_token_impl_token_gt_s.rs.md): `GtS` token execution logic.
- [i.rs](index_cards/src_types_token_impl_token_i.rs.md): `I` token execution logic.
- [i32_const.rs](index_cards/src_types_token_impl_token_i32_const.rs.md): `I32Const` token execution logic.
- [identical.rs](index_cards/src_types_token_impl_token_identical.rs.md): `Identical` token execution logic.
- [iff.rs](index_cards/src_types_token_impl_token_iff.rs.md): `Iff` token execution logic.
- [implies.rs](index_cards/src_types_token_impl_token_implies.rs.md): `Implies` token execution logic.
- [integer.rs](index_cards/src_types_token_impl_token_integer.rs.md): `Integer` token execution logic.
- [k.rs](index_cards/src_types_token_impl_token_k.rs.md): `K` token execution logic.
- [lightning.rs](index_cards/src_types_token_impl_token_lightning.rs.md): `Lightning` token execution logic.
- [local_get.rs](index_cards/src_types_token_impl_token_local_get.rs.md): `LocalGet` token execution logic.
- [local_set.rs](index_cards/src_types_token_impl_token_local_set.rs.md): `LocalSet` token execution logic.
- [mul.rs](index_cards/src_types_token_impl_token_mul.rs.md): `Mul` token execution logic.
- [newline.rs](index_cards/src_types_token_impl_token_newline.rs.md): `Newline` token execution logic.
- [not.rs](index_cards/src_types_token_impl_token_not.rs.md): `Not` token execution logic.
- [not_equals.rs](index_cards/src_types_token_impl_token_not_equals.rs.md): `NotEquals` token execution logic.
- [or.rs](index_cards/src_types_token_impl_token_or.rs.md): `Or` token execution logic.
- [s.rs](index_cards/src_types_token_impl_token_s.rs.md): `S` token execution logic.
- [sparkle.rs](index_cards/src_types_token_impl_token_sparkle.rs.md): `Sparkle` token execution logic.
- [sub.rs](index_cards/src_types_token_impl_token_sub.rs.md): `Sub` token execution logic.
- [true.rs](index_cards/src_types_token_impl_token_true.rs.md): `True` token execution logic.
- [whitespace.rs](index_cards/src_types_token_impl_token_whitespace.rs.md): `Whitespace` token execution logic.

