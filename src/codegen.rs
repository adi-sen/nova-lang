use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    values::{BasicValueEnum, PointerValue},
    types::{BasicType, BasicTypeEnum, BasicMetadataTypeEnum},
    targets::{TargetMachine, Target, InitializationConfig, RelocMode, CodeModel, FileType},
};
use std::collections::HashMap;
use crate::parser::AstNode;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("nova");
        let builder = context.create_builder();
        
        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    pub fn generate(&mut self, ast: &AstNode) -> Result<(), String> {
        match ast {
            AstNode::Program(nodes) => {
                for node in nodes {
                    self.generate_expression(node)?;
                }
                Ok(())
            },
            _ => self.generate_expression(ast),
        }
    }

    fn generate_expression(&mut self, expr: &AstNode) -> Result<(), String> {
        match expr {
            AstNode::Program(nodes) => {
                // code for all nodes in the program/block
                for node in nodes {
                    self.generate_expression(node)?;
                }
                Ok(())
            },
            AstNode::Number(n) => {
                let int_type = self.context.i64_type();
                let _value = int_type.const_int(*n as u64, false);
                Ok(())
            },
            AstNode::Let { name, value, .. } => {
                let val = self.generate_value(value)?;
                let alloca = self.builder.build_alloca(val.get_type(), name)
                    .map_err(|e| format!("Failed to allocate: {:?}", e))?;
                self.builder.build_store(alloca, val)
                    .map_err(|e| format!("Failed to store: {:?}", e))?;
                self.variables.insert(name.clone(), alloca);
                Ok(())
            },
            AstNode::Function { name, params: _, body } => {
                let fn_type = self.context.i32_type().fn_type(&[], false);
                let function = self.module.add_function(name, fn_type, None);
                
                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);
                
                // fn body; program node returned by parse_block
                match &**body {
                    AstNode::Program(statements) => {
                        for stmt in statements {
                            self.generate_expression(stmt)?;
                        }
                    },
                    _ => self.generate_expression(body)?,
                }

                // Only add default return if no explicit return was given
                if !self.builder.get_insert_block().unwrap().get_terminator().is_some() {
                    let default_return = self.context.i32_type().const_int(0, false);
                    self.builder.build_return(Some(&default_return))
                        .map_err(|e| format!("Failed to build default return: {:?}", e))?;
                }

                if function.verify(true) {
                    Ok(())
                } else {
                    Err("Invalid function generated".to_string())
                }
            },
            AstNode::Return(expr) => {
                let return_value = self.generate_value(expr)?;
                self.builder.build_return(Some(&return_value))
                    .map_err(|e| format!("Failed to build return: {:?}", e))?;
                Ok(())
            },
            _ => Ok(()),
        }
    }

    fn generate_value(&self, expr: &AstNode) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            AstNode::Number(n) => {
                let int_type = self.context.i32_type(); // Changed from i64 to i32
                Ok(int_type.const_int(*n as u64, false).into())
            },
            AstNode::Identifier(name) => {
                self.load_variable(name)
            },
            _ => Err("Unsupported expression for value generation".to_string()),
        }
    }

    fn load_variable(&self, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        match self.variables.get(name) {
            Some(ptr) => {
                Ok(self.builder.build_load(self.context.i64_type(), *ptr, name)
                    .map_err(|e| format!("Failed to load variable: {:?}", e))?)
            },
            None => Err(format!("Undefined variable: {}", name)),
        }
    }

    pub fn create_function(&mut self, name: &str, args: &[(&str, BasicTypeEnum<'ctx>)], ret_type: BasicTypeEnum<'ctx>) {
        let arg_types: Vec<_> = args.iter()
            .map(|(_, ty)| ty.as_basic_type_enum().into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        
        let fn_type = ret_type.into_int_type().fn_type(&arg_types, false);
        let function = self.module.add_function(name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // args to variables map
        for (i, (name, _)) in args.iter().enumerate() {
            let arg = function.get_nth_param(i as u32).unwrap();
            let alloca = self.builder.build_alloca(arg.get_type(), name).unwrap();
            self.builder.build_store(alloca, arg).unwrap();
            self.variables.insert(name.to_string(), alloca);
        }
    }

    pub fn write_bitcode_to_file(&self, filename: &str) -> Result<(), String> {
        if self.module.write_bitcode_to_path(std::path::Path::new(filename)) {
            Ok(())
        } else {
            Err("Failed to write bitcode".to_string())
        }
    }

    pub fn write_object_file(&self, filename: &str) -> Result<(), String> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialize target: {:?}", e))?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to get target from triple: {:?}", e))?;

        let machine = target.create_target_machine(
            &triple,
            TargetMachine::get_host_cpu_name().to_str().unwrap(),
            TargetMachine::get_host_cpu_features().to_str().unwrap(),
            inkwell::OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or("Failed to create target machine")?;

        machine.write_to_file(&self.module, FileType::Object, filename.as_ref())
            .map_err(|e| format!("Failed to write object file: {:?}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_codegen() {
        let context = Context::create();
        let mut codegen = CodeGen::new(&context);
        let ast = AstNode::Program(vec![AstNode::Number(42)]);
        assert!(codegen.generate(&ast).is_ok());
    }
}
