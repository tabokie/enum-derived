use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields, Ident, Variant,
};

pub fn expand_derive_rand(input: &DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    // println!("{input:#?}");
    let usr_type = get_attr_value(&input.attrs, "usr").unwrap();

    match input.data {
        Data::Struct(ref data_struct) => {
            expand_derive_rand_struct(&input.ident, &usr_type, data_struct)
        }
        Data::Enum(ref data_enum) => expand_derive_rand_enum(&input.ident, &usr_type, data_enum),
        Data::Union(ref data_union) => {
            expand_derive_rand_union(&input.ident, &usr_type, data_union)
        }
    }
}

fn expand_derive_rand_struct(
    struct_name: &Ident,
    usr_type_name: &proc_macro2::TokenStream,
    data_struct: &DataStruct,
) -> Result<TokenStream, Vec<syn::Error>> {
    // Create rand func for struct
    let struct_ident = quote! { #struct_name };
    let rand_struct_generator =
        build_entity_generator(struct_ident, usr_type_name, &data_struct.fields);

    let expanded = quote! {
        impl ::enum_derived::Rand<#usr_type_name> for #struct_name {
            fn rand<R: ::rand::Rng>(usr: &#usr_type_name, rng: &mut R) -> Self {
                (#rand_struct_generator)(usr, rng)
            }
        }
    };
    Ok(TokenStream::from(expanded))
}

fn expand_derive_rand_enum(
    enum_name: &Ident,
    usr_type_name: &proc_macro2::TokenStream,
    data_enum: &DataEnum,
) -> Result<TokenStream, Vec<syn::Error>> {
    // Cannot be an empty variant
    // TODO is this necessary?
    if data_enum.variants.is_empty() {
        panic!("Enum must have at least one variant defined");
    }

    let variant_gen = |v: &Variant| variant_generator(enum_name, usr_type_name, v);
    let var_rand_funcs = data_enum.variants.iter().map(variant_gen);

    let weights = variant_weights_collector(data_enum.variants.iter().cloned().collect::<Vec<_>>());

    let expanded = quote! {
        impl ::enum_derived::Rand<#usr_type_name> for #enum_name {
            fn rand<R: ::rand::Rng>(usr: &#usr_type_name, rng: &mut R) -> Self {
                use ::rand::{Rng, distributions::{WeightedIndex, Distribution}};

                let mut random_enums: Vec<Box<dyn Fn(&#usr_type_name, &mut R) -> Self>> = vec![#(#var_rand_funcs),*];
                let enum_weights = vec![#(#weights),*];
                let dist = WeightedIndex::new(&enum_weights).unwrap();

                let enum_idx: usize = dist.sample(&mut *rng);
                (*random_enums.swap_remove(enum_idx))(usr, &mut *rng)
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

// Gets the weight cutoff for a variant
fn variant_weights_collector(variants: Vec<Variant>) -> Vec<proc_macro2::TokenStream> {
    variants
        .iter()
        .map(|v| {
            for attr in v.attrs.iter() {
                if attr.path.get_ident().unwrap() == "weight" {
                    return attr.tokens.clone();
                }
            }
            quote! { 1 }
        })
        .collect()
}

/// Creates the rand function for a vartiant
fn variant_generator(
    enum_name: &Ident,
    usr_type_name: &proc_macro2::TokenStream,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let variant_generator = match get_attr_value(&variant.attrs, "custom_rand") {
        Some(f) => f,
        None => {
            let var_ident = &variant.ident;

            let full_variant_ident = quote! { #enum_name::#var_ident };
            build_entity_generator(full_variant_ident, usr_type_name, &variant.fields)
        }
    };

    quote! {
        ::std::boxed::Box::new(|usr, rng| {
            (#variant_generator)(usr, rng)
        })
    }
}

// Returns the generating function
fn build_entity_generator(
    entity_name: proc_macro2::TokenStream,
    usr_type_name: &proc_macro2::TokenStream,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => {
            quote! { |_usr, _rng| #entity_name }
        }
        Fields::Unnamed(unnamed_fields) => {
            let fields_rand_generators = unnamed_fields
                .unnamed
                .iter()
                .map(|f| get_field_generator(usr_type_name, f));
            quote! { |usr: &#usr_type_name, rng: &mut R| #entity_name(#(#fields_rand_generators(usr, &mut *rng)),*) }
        }
        Fields::Named(named_fields) => {
            let fields_ident = named_fields.named.iter().map(|f| f.ident.clone().unwrap());
            let fields_rand_generators = named_fields
                .named
                .iter()
                .map(|f| get_field_generator(usr_type_name, f));

            quote! { |usr: &#usr_type_name, rng: &mut R| #entity_name { #(#fields_ident: #fields_rand_generators(usr, &mut *rng)),* } }
        }
    }
}

fn get_field_generator(
    usr_type_name: &proc_macro2::TokenStream,
    field: &Field,
) -> proc_macro2::TokenStream {
    if let Some(ts) = get_attr_value(&field.attrs, "custom_rand") {
        return quote! {
            (|_usr, rng| #ts(rng))
        };
    }
    if let Some(ts) = get_attr_value(&field.attrs, "custom_rand_member") {
        return quote! {
            (|usr: &#usr_type_name, rng| usr.#ts(rng))
        };
    }
    let field_type = &field.ty;
    quote! {
        <#field_type as ::enum_derived::Rand>::rand
    }
}

fn get_attr_value(attrs: &[Attribute], name: &str) -> Option<proc_macro2::TokenStream> {
    for attr in attrs.iter() {
        let Some(ident) = attr.path.get_ident() else {
            continue;
        };
        // Allow for custom over ride functions to be used
        if ident == name {
            let Ok(value_func) = attr.parse_args::<Ident>() else {
                continue;
            };
            return Some(quote! {
                #value_func
            });
        }
    }
    None
}

fn expand_derive_rand_union(
    _union_name: &Ident,
    _usr_type_name: &proc_macro2::TokenStream,
    _data_union: &DataUnion,
) -> Result<TokenStream, Vec<syn::Error>> {
    panic!("Union types are not supported")
}
