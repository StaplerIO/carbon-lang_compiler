use std::io::Write;
use lazy_static::lazy_static;

use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::function_block::build_function_command;
use crate::parser::decorator::decorate_token;
use crate::parser::pipeline::build_whole_file;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::utils::identifier::Identifier;

lazy_static! {
    static ref FILE_CONTENT: &'static str = r#"
		link os;
		link std;

        group Arc {
			field number foo(get, set);
			field str bar(get);

			method run()[none];
			method suspend()[number];

			func New()[Arc];
		}

		impl Arc {
			default foo = 2;
			default bar = "StaplerIO";

			field bar get {
				return bar;
			}

			method run()[none] { foo = foo + 1; }
			method suspend()[number] { return foo - 1; }

			func New()[Arc] { decl var number i_foo; }
		}

		decl func main(number foo)[number] {
			decl var number bar;
			bar = 2;
			decl var number result;
			result = arc::target(foo * bar);
			call f1();
			decl var str test;
			test = "Hello, world!";
			return result;
		}

		decl func arc::target(number v1)[number] {
			decl var number bar;
			bar = 2;
			decl var number result;
			result = v1 + bar;
			return result;
		}

		decl func f1()[none] {
			decl var number v2;
			v2 = 37413;
			if (v2 > 30000) {
				v2 = 1201;
			}

			return;
		}

		decl func f2()[none] {
            while (234 > 123) {
                if (234 > 245) {
                    break;
                } else {
                    continue;
                }
            }

            return;
        }
        "#;
}

#[test]
fn code_file() {
    let tokens = decorate_token(tokenize(&FILE_CONTENT, true).unwrap()).0;
    let structure = build_whole_file(tokens, Identifier::single("main"))
        .ok()
        .unwrap();

    let metadata = PackageMetadata {
        data_slot_alignment: 2,
        data_alignment: 4,
        package_type: 2,
        global_command_offset: 5,
        domain_layer_count_alignment: 2,
        address_alignment: 4,
    };

    let mut result = RelocatableCommandList::new();
    result.function_table = structure.export_function_table();

    for func in &structure.functions {
        result.function_table.iter_mut().find(|f| f.name == func.declarator.identifier).unwrap().relocated_entry_address = result.commands.len();
        result.combine(build_function_command(func, &metadata));
    }

    result.calculate_ref_to_target();
    result.apply_relocation(metadata.address_alignment);

    let mut file = std::fs::File::create("F:\\test.cbp").unwrap();
    file.write_all(metadata.serialize().as_slice()).unwrap();
    file.write_all(result.commands.as_slice()).unwrap();

    println!("{}", itertools::Itertools::join(&mut result.commands.iter(), ", "));
}
