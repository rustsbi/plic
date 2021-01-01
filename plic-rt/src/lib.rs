// Ref: cortex-m-rt-macros
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashSet;
use std::iter;
use syn::{
    parse, spanned::Spanned, AttrStyle, Attribute, FnArg, Ident, Item, ItemFn,
    ItemStatic, ReturnType, Stmt, Type, Visibility,
};

use std::sync::Once;

static HAS_TRAP_HANDLER: Once = Once::new();

#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f: ItemFn = syn::parse(input).expect("`#[interrupt]` must be applied to a function");

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "`#[interrupt]` attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let ident = f.sig.ident.clone();
    let ident_s = ident.to_string();

    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[interrupt]` handlers must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let (statics, stmts) = match extract_static_muts(f.block.stmts.iter().cloned()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    f.sig.ident = Ident::new(&format!("__plic_rt_{}", f.sig.ident), Span::call_site());
    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;
        syn::parse::<FnArg>(quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &mut #ty).into())
            .unwrap()
    }));
    f.block.stmts = iter::once(
        syn::parse2(quote! {{
            // Check that this interrupt actually exists
            Interrupt::#ident;
        }})
        .unwrap(),
    )
    .chain(stmts)
    .collect();

    let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
    let ident = &f.sig.ident;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    &mut #ident
                }
            }
        })
        .collect::<Vec<_>>();

    let mut once_trap_handler = quote!();
    
    HAS_TRAP_HANDLER.call_once(|| {
        once_trap_handler = gen_once_trap_handler();
    });

    quote!(
        #once_trap_handler

        #[doc(hidden)]
        #[export_name = #ident_s]
        pub unsafe extern "C" fn #tramp_ident() {
            #ident(
                #(#resource_args),*
            )
        }

        #f
    )
    .into()
}

fn gen_once_trap_handler() -> proc_macro2::TokenStream {
    quote!(
#[export_name = "MachineExternal"]
fn __plic_rt_machine_external_handler() {
    use riscv::register::{mhartid, mie};

    let hart_id = mhartid::read();
    let threshold = pac::PLIC::get_threshold(hart_id);
    let irq = pac::PLIC::claim(hart_id).unwrap();
    let prio = pac::PLIC::get_priority(irq);
    unsafe { 
        pac::PLIC::set_threshold(hart_id, prio);
        mie::clear_msoft();
        mie::clear_mtimer();
    }
    
    let irq_index = irq as usize;
    unsafe { 
        (pac::__INTERRUPTS[irq_index].handler)()
    };

    unsafe { 
        mie::set_mtimer();
        mie::set_msoft();
        pac::PLIC::set_threshold(hart_id, threshold);
    }
    pac::PLIC::complete(hart_id, irq);
}
    )
}

// Ref: cortex-m-rt-macros
/// Extracts `static mut` vars from the beginning of the given statements
fn extract_static_muts(
    stmts: impl IntoIterator<Item = Stmt>,
) -> Result<(Vec<ItemStatic>, Vec<Stmt>), parse::Error> {
    let mut istmts = stmts.into_iter();

    let mut seen = HashSet::new();
    let mut statics = vec![];
    let mut stmts = vec![];
    while let Some(stmt) = istmts.next() {
        match stmt {
            Stmt::Item(Item::Static(var)) => {
                if var.mutability.is_some() {
                    if seen.contains(&var.ident) {
                        return Err(parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ));
                    }

                    seen.insert(var.ident.clone());
                    statics.push(var);
                } else {
                    stmts.push(Stmt::Item(Item::Static(var)));
                }
            }
            _ => {
                stmts.push(stmt);
                break;
            }
        }
    }

    stmts.extend(istmts);

    Ok((statics, stmts))
}

fn extract_cfgs(attrs: Vec<Attribute>) -> (Vec<Attribute>, Vec<Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

/// Returns `true` if `attr.path` matches `name`
fn eq(attr: &Attribute, name: &str) -> bool {
    attr.style == AttrStyle::Outer && attr.path.is_ident(name)
}
