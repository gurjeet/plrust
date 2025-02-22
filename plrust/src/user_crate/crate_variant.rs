/*
Copyright 2021-2023 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the PostgreSQL license that can be found in the LICENSE.md file.
*/

use crate::{user_crate::oid_to_syn_type, PlRustError};
use eyre::WrapErr;
use pgx::PgOid;
use quote::quote;

/// What kind of PL/Rust function must be built

/// Includes arguments and return type, if applicable
/// Used to create the source code that is built
#[must_use]
#[derive(Clone)]
pub(crate) enum CrateVariant {
    Function {
        arguments: Vec<syn::FnArg>,
        return_type: syn::Type,
        #[allow(dead_code)] // For debugging
        return_oid: PgOid,
        #[allow(dead_code)] // For debugging
        return_set: bool,
        #[allow(dead_code)] // For debugging
        is_strict: bool,
    },
    Trigger,
}

impl CrateVariant {
    #[tracing::instrument(level = "debug", skip_all)]
    pub(crate) fn function(
        argument_oids_and_names: Vec<(PgOid, syn::Ident)>,
        return_oid: PgOid,
        return_set: bool,
        is_strict: bool,
    ) -> eyre::Result<Self> {
        let mut arguments = Vec::new();
        for (argument_oid, arg_name) in argument_oids_and_names.into_iter() {
            let rust_type: syn::Type = {
                let bare = oid_to_syn_type(&argument_oid, false)?;
                match is_strict {
                    true => bare,
                    false => syn::parse2(quote! {
                        Option<#bare>
                    })
                    .wrap_err("Wrapping argument type")?,
                }
            };

            let rust_pat_type: syn::FnArg = syn::parse2(quote! {
                #arg_name: #rust_type
            })
            .map_err(PlRustError::Parse)
            .wrap_err("Making argument pattern type")?;
            arguments.push(rust_pat_type);
        }

        let return_type: syn::Type = {
            let bare = oid_to_syn_type(&return_oid, true)?;
            match return_set {
                true => syn::parse2(quote! { ::std::result::Result<Option<::pgx::iter::SetOfIterator<'a, Option<#bare>>>, Box<dyn std::error::Error + Send + Sync + 'static>> })
                    .wrap_err("Wrapping return type")?,
                false => syn::parse2(
                    quote! { ::std::result::Result<Option<#bare>, Box<dyn std::error::Error + Send + Sync + 'static>> },
                )
                .wrap_err("Wrapping return type")?,
            }
        };

        Ok(Self::Function {
            arguments,
            return_oid,
            return_type,
            return_set,
            is_strict,
        })
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub(crate) fn trigger() -> Self {
        Self::Trigger
    }
}
