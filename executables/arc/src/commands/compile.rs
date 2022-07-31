use std::{fs, io::Write};

use carbon_lang_compiler::{
    lexer::tokenize::tokenize,
    package_generator::{command_builder::function_block::build_function_command, utils::align_array_width},
    parser::{decorator::decorate_token, pipeline::build_whole_file},
    shared::package_generation::{
        package_descriptor::PackageMetadata, relocation_reference::{RelocatableCommandList, RelocationReferenceType},
    },
};
use chrono::Local;
use console::style;

use crate::{
    managers::logging::{log_error, log_info},
    models::command_args::CompileCommandArgs,
};

pub fn compile_package(args: CompileCommandArgs) {
    // Calculate procedure time
    let time_start = Local::now();

    // Find out whether we are going to compile a directory or files
    if args.input_path.is_dir() {
        // Fill code later
    } else {
        let file_content = fs::read_to_string(&args.input_path);
        if file_content.is_ok() {
            let tokens_result = tokenize(file_content.unwrap().as_str(), true);

            // Handle errors
            if tokens_result.is_err() {
                let err = tokens_result.unwrap_err();
                for item in err.issues {
                    log_error(format!("({}) {}", item.code, item.detail).as_str());
                }
                return;
            }

            let tokens = tokens_result.unwrap();

            log_info(format!("Found {} tokens", tokens.len()).as_str());

            let decorated_tokens = decorate_token(tokens);
            let tree_result = build_whole_file(decorated_tokens, args.entry_function);

            let metadata = PackageMetadata {
                data_slot_alignment: 2,
                data_alignment: 8,
                package_type: 0,
                domain_layer_count_alignment: 2,
                address_alignment: 8,
                global_command_offset: 5,
            };

            if tree_result.is_ok() {
                let tree = tree_result.unwrap();

                let mut output = RelocatableCommandList::new();
                for func in &tree.functions {
                    output.combine(build_function_command(func, &metadata));
                }

                // Reserve location for entry point
                let prefix_len = metadata.serialize().len();
                output.append_commands(align_array_width(vec![0x00], metadata.address_alignment));

                output.calculate_ref_to_target(prefix_len);
                output.apply_relocation(metadata.address_alignment);

                // Place entry_point
                let addr_u8_vec = align_array_width((output.descriptors.references.iter()
                                                           .find(|&p| p.ref_type == RelocationReferenceType::FunctionEntrance(tree.entry_point.clone()))
                                                           .unwrap()
                                                           .command_array_position + prefix_len)
                                                                    .to_be_bytes()
                                                                    .to_vec(),
                                                    metadata.address_alignment);
                output.commands.splice(prefix_len..(prefix_len + metadata.address_alignment as usize), addr_u8_vec);

                let mut output_file = fs::File::create(&args.output_path).unwrap();
                output_file
                    .write_all(metadata.serialize().as_slice())
                    .unwrap();
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
                log_error("Errors occurred during package compilation");
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
