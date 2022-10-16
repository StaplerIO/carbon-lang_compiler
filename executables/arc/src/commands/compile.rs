use std::{fs, io::Write};

use chrono::Local;
use console::style;

use carbon_lang_compiler::{
    package_generator::{command_builder::function_block::build_function_command, utils::align_array_width},
    shared::package_generation::{
        package_descriptor::PackageMetadata, relocation_reference::{RelocatableCommandList, RelocationReferenceType},
    },
};

use crate::{
    managers::logging::{log_error, log_info, log_trace},
    models::command_args::CompileCommandArgs,
};
use crate::managers::compilation::{parse_tokens, token_conversion};

pub fn compile_package(args: CompileCommandArgs) {
    // Calculate procedure time
    let time_start = Local::now();

    // Find out whether we are going to compile a directory or files
    if args.input_path.is_dir() {
        // TODO: Add directory builder
    } else {
        let file_content = fs::read_to_string(&args.input_path);
        if file_content.is_ok() {
            let tokens_result = token_conversion(file_content.unwrap().as_str());
            if tokens_result.is_none() {
                log_error("Failed to pass lexical analysis, compilation aborted");
                return;
            } else {
                log_info("Lexical analysis passed");
            }

            let tokens = tokens_result.unwrap();

            log_trace(format!("Found {} tokens", tokens.0.len()).as_str());

            let decorated_tokens = tokens.0;
            let string_pool = tokens.1;

            let tree_result = parse_tokens(decorated_tokens, Some(args.entry_function));
            if tree_result.is_none() {
                log_error("Failed to pass token parsing, compilation aborted!");
                return;
            } else {
                log_info("Token parsing passed");
            }

            let metadata = PackageMetadata {
                data_slot_alignment: 2,
                data_alignment: 8,
                package_type: 0,
                domain_layer_count_alignment: 2,
                address_alignment: 8,
                global_command_offset: 5,
            };

            if tree_result.is_some() {
                let tree = tree_result.unwrap();

                let mut output = RelocatableCommandList::new();
                output.string_pool = string_pool;
                // Place metadata
                let serialized_metadata = metadata.serialize();
                output.append_commands(serialized_metadata);
                let prefix_len = output.commands.len();

                // Reserve space for entry point if it is an executable
                if metadata.package_type == 0 {
                    output.append_commands(align_array_width(&vec![0x00], metadata.address_alignment));
                }

                // Place string pool
                let string_pool_command = output.generate_string_pool(metadata.data_slot_alignment);
                output.append_commands(string_pool_command);

                // Place functions
                for func in &tree.functions {
                    output.combine(build_function_command(func, &metadata));
                }

                output.calculate_ref_to_target();
                output.apply_relocation(metadata.address_alignment);

                // Place entry_point
                let entry_function = output.descriptors.references.iter()
                                           .find(|&p| p.ref_type == RelocationReferenceType::FunctionEntrance(tree.entry_point.clone()))
                                           .unwrap();
                let addr_u8_vec = align_array_width(entry_function.command_array_position.to_be_bytes().to_vec().as_ref(), metadata.address_alignment);
                output.commands.splice(prefix_len..(prefix_len + metadata.address_alignment as usize), addr_u8_vec);

                let mut output_file = fs::File::create(&args.output_path).unwrap();
                output_file.write_all(&output.commands.as_slice()).unwrap();

                let time_spanned = Local::now() - time_start;
                log_info(
                    format!(
                        "Compilation finished in {}s",
                        (time_spanned.num_milliseconds() as f64 / 1000 as f64)
                    )
                        .as_str(),
                );
            } else {
                log_error("Errors occurred during code generation");
            }
        } else {
            log_error(
                format!(
                    "{}: couldn't open file \"{}\"",
                    style("Error").red(),
                    style(args.input_path.as_path().to_str().unwrap()).green()
                )
                    .as_str(),
            );
        }
    }
}
