use std::{
    fs::{self, File},
    io::Write,
    process::{Command, Stdio},
};

use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use syn::{Ident, Type};

#[derive(Debug, Serialize, Deserialize)]
struct Idl {
    version: String,
    name: String,
    instructions: Vec<Instruction>,
    types: Vec<TypeDef>,
    accounts: Vec<AccountDef>,
    events: Vec<EventDef>,
    errors: Vec<ErrorDef>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Instruction {
    name: String,
    accounts: Vec<Account>,
    args: Vec<Arg>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    name: String,
    #[serde(rename = "isMut")]
    is_mut: bool,
    #[serde(rename = "isSigner")]
    is_signer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Arg {
    name: String,
    #[serde(rename = "type")]
    arg_type: ArgType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ArgType {
    Simple(String),
    Defined { defined: String },
    Array { array: (Box<ArgType>, usize) },
    Option { option: Box<ArgType> },
    Vec { vec: Box<ArgType> },
}

impl ArgType {
    fn to_rust_type(&self) -> String {
        match self {
            ArgType::Simple(t) => {
                // special cases likely from manual edits to IDL
                if t == "publicKey" {
                    "Pubkey".to_string()
                } else if t == "bytes" {
                    "Vec<u8>".to_string()
                } else {
                    t.clone()
                }
            }
            ArgType::Defined { defined } => defined.clone(),
            ArgType::Array { array: (t, len) } => format!("[{}; {}]", t.to_rust_type(), len),
            ArgType::Option { option } => format!("Option<{}>", option.to_rust_type()),
            ArgType::Vec { vec } => format!("Vec<{}>", vec.to_rust_type()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TypeDef {
    name: String,
    #[serde(rename = "type")]
    type_def: TypeData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
enum TypeData {
    #[serde(rename = "struct")]
    Struct { fields: Vec<StructField> },
    #[serde(rename = "enum")]
    Enum { variants: Vec<EnumVariant> },
}

#[derive(Debug, Serialize, Deserialize)]
struct StructField {
    name: String,
    #[serde(rename = "type")]
    field_type: ArgType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EnumVariant {
    // NB: this must come before `Simple` (harder match -> easiest match)
    Complex {
        name: String,
        fields: Vec<StructField>,
    },
    Simple {
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountDef {
    name: String,
    #[serde(rename = "type")]
    account_type: AccountType,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountType {
    kind: String, // Typically "struct"
    fields: Vec<StructField>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorDef {
    code: u32,
    name: String,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventDef {
    name: String,
    fields: Vec<EventField>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventField {
    name: String,
    #[serde(rename = "type")]
    field_type: ArgType,
    index: bool,
}

fn generate_idl_types(idl: &Idl) -> String {
    let mut instructions_tokens = quote! {};
    let mut types_tokens = quote! {};
    let mut accounts_tokens = quote! {};
    let mut errors_tokens = quote! {};
    let mut events_tokens = quote! {};

    // Generate enums and structs from the types section
    for type_def in &idl.types {
        let type_name = Ident::new(
            &capitalize_first_letter(&type_def.name),
            proc_macro2::Span::call_site(),
        );
        let type_tokens = match &type_def.type_def {
            TypeData::Enum { variants } => {
                let has_complex_variant = variants.iter().any(|v| match v {
                    EnumVariant::Complex { .. } => true,
                    _ => false,
                });

                let variant_tokens =
                    variants
                        .iter()
                        .enumerate()
                        .map(|(i, variant)| match variant {
                            EnumVariant::Simple { name } => {
                                let variant_name = Ident::new(name, proc_macro2::Span::call_site());
                                if i == 0 {
                                    quote! {
                                        #[default]
                                        #variant_name,
                                    }
                                } else {
                                    quote! {
                                        #variant_name,
                                    }
                                }
                            }
                            EnumVariant::Complex { name, fields } => {
                                let variant_name = Ident::new(name, proc_macro2::Span::call_site());
                                let field_tokens = fields.iter().map(|field| {
                                    let field_name = Ident::new(
                                        &to_snake_case(&field.name),
                                        proc_macro2::Span::call_site(),
                                    );
                                    let field_type: Type =
                                        syn::parse_str(&field.field_type.to_rust_type()).unwrap();
                                    quote! {
                                        #field_name: #field_type,
                                    }
                                });
                                quote! {
                                    #variant_name {
                                        #(#field_tokens)*
                                    },
                                }
                            }
                        });

                if has_complex_variant {
                    quote! {
                        #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Debug, PartialEq)]
                        pub enum #type_name {
                            #(#variant_tokens)*
                        }
                    }
                } else {
                    // TODO: need more work to derive 'Default' on complex enums, not currently required
                    quote! {
                        #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq)]
                        pub enum #type_name {
                            #(#variant_tokens)*
                        }
                    }
                }
            }
            TypeData::Struct { fields } => {
                let struct_name =
                    Ident::new(type_def.name.as_str(), proc_macro2::Span::call_site());
                let struct_fields = fields.iter().map(|field| {
                    let field_name =
                        Ident::new(&to_snake_case(&field.name), proc_macro2::Span::call_site());
                    let field_type: syn::Type =
                        syn::parse_str(&field.field_type.to_rust_type()).unwrap();
                    quote! {
                        pub #field_name: #field_type,
                    }
                });

                quote! {
                    #[repr(C)]
                    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq)]
                    pub struct #struct_name {
                        #(#struct_fields)*
                    }
                }
            }
        };

        types_tokens = quote! {
            #types_tokens
            #type_tokens
        };
    }

    // Generate structs for accounts section
    for account in &idl.accounts {
        let struct_name = Ident::new(&account.name, proc_macro2::Span::call_site());

        let struct_fields = account.account_type.fields.iter().map(|field| {
            let field_name =
                Ident::new(&to_snake_case(&field.name), proc_macro2::Span::call_site());

            let mut field_type: Type = syn::parse_str(&field.field_type.to_rust_type()).unwrap();
            // workaround for padding types preventing outertype from deriving 'Default'
            if field_name == "padding" {
                if let ArgType::Array { array: (_t, len) } = &field.field_type {
                    field_type = syn::parse_str(&format!("Padding<{len}>")).unwrap();
                }
            }

            quote! {
                pub #field_name: #field_type,
            }
        });

        let discriminator: TokenStream = format!("{:?}", sighash("account", &account.name))
            .parse()
            .unwrap();
        let struct_def = quote! {
            #[repr(C)]
            #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq)]
            pub struct #struct_name {
                #(#struct_fields)*
            }
            #[automatically_derived]
            impl anchor_lang::Discriminator for #struct_name {
                const DISCRIMINATOR: [u8; 8] = #discriminator;
            }
            #[automatically_derived]
            unsafe impl anchor_lang::__private::bytemuck::Pod for #struct_name {}
            #[automatically_derived]
            unsafe impl anchor_lang::__private::bytemuck::Zeroable for #struct_name {}
            #[automatically_derived]
            impl anchor_lang::ZeroCopy for #struct_name {}
            #[automatically_derived]
            impl anchor_lang::AccountSerialize for #struct_name {
                fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                    if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }

                    if AnchorSerialize::serialize(self, writer).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }

                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::AccountDeserialize for #struct_name {
                fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                    let given_disc = &buf[..8];
                    if Self::DISCRIMINATOR != given_disc {
                        return Err(anchor_lang::error!(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch));
                    }
                    Self::try_deserialize_unchecked(buf)
                }

                fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                    let mut data: &[u8] = &buf[8..];
                    AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
                }
            }
        };

        accounts_tokens = quote! {
            #accounts_tokens
            #struct_def
        };
    }

    // Generate structs for instructions
    for instr in &idl.instructions {
        let name = capitalize_first_letter(&instr.name);
        let fn_name = to_snake_case(&instr.name);
        let struct_name = Ident::new(&name, proc_macro2::Span::call_site());
        let fields = instr.args.iter().map(|arg| {
            let field_name = Ident::new(&to_snake_case(&arg.name), proc_macro2::Span::call_site());
            let field_type: Type = syn::parse_str(&arg.arg_type.to_rust_type()).unwrap();
            quote! {
                pub #field_name: #field_type,
            }
        });
        // https://github.com/coral-xyz/anchor/blob/e48e7e60a64de77d878cdb063965cf125bec741a/lang/syn/src/codegen/program/instruction.rs#L32
        let discriminator: TokenStream =
            format!("{:?}", sighash("global", &fn_name)).parse().unwrap();
        let struct_def = quote! {
            #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
            pub struct #struct_name {
                #(#fields)*
            }
            #[automatically_derived]
            impl anchor_lang::Discriminator for #struct_name {
                const DISCRIMINATOR: [u8; 8] = #discriminator;
            }
            #[automatically_derived]
            impl anchor_lang::InstructionData for #struct_name {}
        };

        instructions_tokens = quote! {
            #instructions_tokens
            #struct_def
        };

        let accounts = instr.accounts.iter().map(|acc| {
            let account_name =
                Ident::new(&to_snake_case(&acc.name), proc_macro2::Span::call_site());
            quote! {
                pub #account_name: Pubkey,
            }
        });

        let to_account_metas = instr.accounts.iter().map(|acc| {
            let account_name_str = to_snake_case(&acc.name);
            let account_name =
                Ident::new(&account_name_str, proc_macro2::Span::call_site());
            let is_mut: TokenStream = acc.is_mut.to_string().parse().unwrap();
            let is_signer: TokenStream = acc.is_signer.to_string().parse().unwrap();
            quote! {
                AccountMeta { pubkey: self.#account_name, is_signer: #is_signer, is_writable: #is_mut },
            }
        });

        let discriminator: TokenStream =
            format!("{:?}", sighash("account", &name)).parse().unwrap();
        let account_struct_def = quote! {
            #[repr(C)]
            #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
            pub struct #struct_name {
                #(#accounts)*
            }
            #[automatically_derived]
            impl anchor_lang::Discriminator for #struct_name {
                const DISCRIMINATOR: [u8; 8] = #discriminator;
            }
            #[automatically_derived]
            unsafe impl anchor_lang::__private::bytemuck::Pod for #struct_name {}
            #[automatically_derived]
            unsafe impl anchor_lang::__private::bytemuck::Zeroable for #struct_name {}
            #[automatically_derived]
            impl anchor_lang::ZeroCopy for #struct_name {}
            #[automatically_derived]
            impl anchor_lang::InstructionData for #struct_name {}
            #[automatically_derived]
            impl ToAccountMetas for #struct_name {
                fn to_account_metas(
                    &self,
                ) -> Vec<AccountMeta> {
                   vec![
                        #(#to_account_metas)*
                    ]
                }
            }
            #[automatically_derived]
            impl anchor_lang::AccountSerialize for #struct_name {
                fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                    if writer.write_all(&Self::DISCRIMINATOR).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }

                    if AnchorSerialize::serialize(self, writer).is_err() {
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                    }

                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::AccountDeserialize for #struct_name {
                fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                    let given_disc = &buf[..8];
                    if Self::DISCRIMINATOR != given_disc {
                        return Err(anchor_lang::error!(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch));
                    }
                    Self::try_deserialize_unchecked(buf)
                }

                fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                    let mut data: &[u8] = &buf[8..];
                    AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
                }
            }
        };

        accounts_tokens = quote! {
            #accounts_tokens
            #account_struct_def
        };
    }

    // Generate enum for errors
    let error_variants = idl.errors.iter().map(|error| {
        let variant_name = Ident::new(&error.name, proc_macro2::Span::call_site());
        let error_msg = &error.msg;
        quote! {
            #[msg(#error_msg)]
            #variant_name,
        }
    });

    let error_enum = quote! {
        #[derive(PartialEq)]
        #[error_code]
        pub enum ErrorCode {
            #(#error_variants)*
        }
    };

    errors_tokens = quote! {
        #errors_tokens
        #error_enum
    };

    // Generate event structs from the events section
    for event in &idl.events {
        let struct_name = Ident::new(&event.name, proc_macro2::Span::call_site());
        let fields = event.fields.iter().map(|field| {
            let field_name =
                Ident::new(&to_snake_case(&field.name), proc_macro2::Span::call_site());
            let field_type: Type = syn::parse_str(&field.field_type.to_rust_type()).unwrap();
            quote! {
                pub #field_name: #field_type,
            }
        });

        let struct_def = quote! {
            //#[derive(InitSpace)]
            #[event]
            pub struct #struct_name {
                #(#fields)*
            }
        };

        events_tokens = quote! {
            #events_tokens
            #struct_def
        };
    }

    // Wrap generated code in modules with necessary imports
    let output = quote! {
        #![allow(unused_imports)]
        //!
        //! Auto-generated IDL types, manual edits do not persist (see `crates/drift-idl-gen`)
        //!
        use anchor_lang::{prelude::{account, AnchorSerialize, AnchorDeserialize, InitSpace, event, error_code, msg, borsh::{self}}, Discriminator};
        // use solana-sdk PUbkey, the vendored anchor-lang Pubkey maybe behind
        use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
        use self::traits::ToAccountMetas;

        pub mod traits {
            use solana_sdk::instruction::AccountMeta;

            /// This is distinct from the anchor version of the trait
            /// reimplemented to ensure the types used are from solana crates _not_ the anchor vendored versions which may be lagging behind
            pub trait ToAccountMetas {
                fn to_account_metas(&self) -> Vec<AccountMeta>;
            }
        }

        pub mod instructions {
            use super::{*, types::*};

            #instructions_tokens
        }

        pub mod types {
            use std::ops::Mul;

            use super::*;
            /// backwards compatible u128 deserializing data from rust <=1.76.0 when u/i128 was 8-byte aligned
            /// https://solana.stackexchange.com/questions/7720/using-u128-without-sacrificing-alignment-8
            #[derive(
                Default,
                PartialEq,
                AnchorSerialize,
                AnchorDeserialize,
                Copy,
                Clone,
                bytemuck::Zeroable,
                bytemuck::Pod,
                Debug,
            )]
            #[repr(C)]
            pub struct u128(pub [u8; 16]);
        
            impl u128 {
                /// convert self into the std `u128` type
                pub fn as_u128(&self) -> std::primitive::u128 {
                    std::primitive::u128::from_le_bytes(self.0)
                }
            }
        
            impl From<std::primitive::u128> for self::u128 {
                fn from(value: std::primitive::u128) -> Self {
                    Self(value.to_le_bytes())
                }
            }
        
            /// backwards compatible i128 deserializing data from rust <=1.76.0 when u/i128 was 8-byte aligned
            /// https://solana.stackexchange.com/questions/7720/using-u128-without-sacrificing-alignment-8
            #[derive(
                Default,
                PartialEq,
                AnchorSerialize,
                AnchorDeserialize,
                Copy,
                Clone,
                bytemuck::Zeroable,
                bytemuck::Pod,
                Debug,
            )]
            #[repr(C)]
            pub struct i128(pub [u8; 16]);
        
            impl i128 {
                /// convert self into the std `i128` type
                pub fn as_i128(&self) -> core::primitive::i128 {
                    core::primitive::i128::from_le_bytes(self.0)
                }
            }
        
            impl From<core::primitive::i128> for i128 {
                fn from(value: core::primitive::i128) -> Self {
                    Self(value.to_le_bytes())
                }
            }

            /// wrapper around fixed array types used for padding with `Default` implementation
            #[repr(transparent)]
            #[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq)]
            pub struct Padding<const N: usize>([u8; N]);
            impl<const N: usize> Default for Padding<N> {
                fn default() -> Self {
                    Self([0u8; N])
                }
            }

            impl<const N: usize> std::fmt::Debug for Padding<N> {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    // don't print anything for padding...
                    Ok(())
                }
            }

            impl<const N: usize> anchor_lang::Space for Padding<N> {
                const INIT_SPACE: usize = 8 * N;
            }

            #types_tokens
        }

        pub mod accounts {
            use super::{*, types::*};

            #accounts_tokens
        }

        pub mod errors {
            use super::{*, types::*};

            #errors_tokens
        }

        pub mod events {
            use super::{*, types::*};
            #events_tokens
        }
    };

    output.to_string()
}

fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");
    let mut hasher = sha2::Sha256::default();
    let mut sighash = <[u8; 8]>::default();
    hasher.update(preimage.as_bytes());
    let digest = hasher.finalize();
    sighash.copy_from_slice(&digest.as_slice()[..8]);

    sighash
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake_case.push('_');
            }
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn format_rust_code(code: &str) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rustfmt");

    {
        let stdin = rustfmt.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(code.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = rustfmt
        .wait_with_output()
        .expect("Failed to read rustfmt output");

    String::from_utf8(output.stdout).expect("rustfmt output is not valid UTF-8")
}

/// Generate rust types from IDL json
/// Output is emitted to `<NAME>_idl.rs`
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the JSON file
    let data = fs::read_to_string("drift.json")?;
    let idl: Idl = serde_json::from_str(&data)?;

    // Generate Rust structs organized into modules
    let rust_idl_types = format_rust_code(&generate_idl_types(&idl));

    // Write the generated Rust code to the file
    let file_name = format!("{}_idl.rs", idl.name);
    let mut file = File::create(&file_name)?;
    file.write_all(rust_idl_types.as_bytes())?;

    Ok(())
}
