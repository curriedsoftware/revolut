use openapiv3::{
    AdditionalProperties, ArrayType, ObjectType, OpenAPI, ReferenceOr, Schema as OASchema,
    SchemaKind as OASchemaKind, StringType, Type,
};

use crate::naming::{to_type_name, to_variant_name};
use crate::types::*;

/// Parse an OpenAPI spec file and extract all schemas.
///
/// We first try strict parsing with `openapiv3`. If that fails (e.g. due to
/// invalid defaults in path parameters), we fall back to a lenient approach:
/// extract the `components` section from the YAML, reconstruct a minimal
/// OpenAPI document with only that section, and parse that.
pub fn parse_spec(path: &str) -> Vec<Schema> {
    let contents = std::fs::read_to_string(path).expect("Failed to read spec file");

    let spec: OpenAPI = match serde_yaml::from_str(&contents) {
        Ok(spec) => spec,
        Err(e) => {
            eprintln!(
                "Strict parse failed ({e}), falling back to lenient components-only parse..."
            );
            parse_components_only(&contents)
        }
    };

    let mut schemas = Vec::new();

    if let Some(components) = &spec.components {
        for (name, schema_ref) in &components.schemas {
            match schema_ref {
                ReferenceOr::Item(schema) => {
                    if let Some(s) = parse_schema(name, schema, &spec) {
                        schemas.push(s);
                    }
                }
                ReferenceOr::Reference { reference } => {
                    // Top-level $ref — emit as a type alias
                    let ref_type = resolve_ref_type(reference, &spec);
                    let type_name = to_type_name(name);
                    schemas.push(Schema {
                        name: type_name,
                        kind: SchemaKind::TypeAlias(ref_type.to_rust_type()),
                    });
                }
            }
        }
    }

    schemas
}

/// Extract only the components section from the YAML and parse it as a
/// minimal OpenAPI document. Strips `default` and `example` keys that
/// often contain invalid values (like `"now + 7 days"` for a number field).
fn parse_components_only(contents: &str) -> OpenAPI {
    let mut value: serde_yaml::Value =
        serde_yaml::from_str(contents).expect("Failed to parse YAML at all");

    let components = value
        .get_mut("components")
        .expect("No components section found in spec")
        .clone();

    // Strip default/example values that may be invalid
    let mut cleaned_components = strip_problematic_keys(components);

    // Keep only the schemas section from components — parameters, responses, etc.
    // can have issues that don't affect our code generation.
    if let serde_yaml::Value::Mapping(ref mut m) = cleaned_components {
        let schemas = m
            .get(&serde_yaml::Value::String("schemas".to_string()))
            .cloned();
        m.clear();
        if let Some(schemas) = schemas {
            // Fix schemas that have both `type` and `$ref` at the same level.
            // OpenAPI 3.0 doesn't support this, but some specs use it. We convert
            // these to just `$ref` (type alias).
            let fixed_schemas = fix_type_plus_ref(schemas);
            m.insert(
                serde_yaml::Value::String("schemas".to_string()),
                fixed_schemas,
            );
        }
    }

    let info_raw: serde_yaml::Value =
        serde_yaml::from_str("title: unknown\nversion: '0.0.0'").unwrap();

    let minimal = serde_yaml::Value::Mapping({
        let mut m = serde_yaml::Mapping::new();
        m.insert(
            serde_yaml::Value::String("openapi".to_string()),
            serde_yaml::Value::String("3.0.0".to_string()),
        );
        m.insert(serde_yaml::Value::String("info".to_string()), info_raw);
        m.insert(
            serde_yaml::Value::String("paths".to_string()),
            serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
        );
        m.insert(
            serde_yaml::Value::String("components".to_string()),
            cleaned_components,
        );
        m
    });

    let yaml_str = serde_yaml::to_string(&minimal).expect("Failed to serialize minimal spec");
    serde_yaml::from_str(&yaml_str).expect("Failed to parse minimal OpenAPI spec")
}

/// Fix schemas that have both `type` and `$ref` at the same level.
/// These are converted to pure `$ref` schemas (the `type` is redundant).
fn fix_type_plus_ref(value: serde_yaml::Value) -> serde_yaml::Value {
    match value {
        serde_yaml::Value::Mapping(m) => {
            let fixed: serde_yaml::Mapping = m
                .into_iter()
                .map(|(k, v)| {
                    let v = fix_single_schema_type_ref(v);
                    (k, fix_type_plus_ref(v))
                })
                .collect();
            serde_yaml::Value::Mapping(fixed)
        }
        serde_yaml::Value::Sequence(seq) => {
            serde_yaml::Value::Sequence(seq.into_iter().map(fix_type_plus_ref).collect())
        }
        other => other,
    }
}

/// If a mapping has both `type` and `$ref` at the same level, remove `type`
/// and `properties` so the `$ref` takes precedence.
fn fix_single_schema_type_ref(value: serde_yaml::Value) -> serde_yaml::Value {
    if let serde_yaml::Value::Mapping(ref m) = value {
        let has_ref = m.contains_key(&serde_yaml::Value::String("$ref".to_string()));
        let has_type = m.contains_key(&serde_yaml::Value::String("type".to_string()));
        if has_ref && has_type {
            let mut m = m.clone();
            m.remove(&serde_yaml::Value::String("type".to_string()));
            m.remove(&serde_yaml::Value::String("properties".to_string()));
            return serde_yaml::Value::Mapping(m);
        }

        // If a mapping has both `type: object` with `properties` and `anyOf`/`oneOf`
        // where the anyOf/oneOf only contains `required` constraints, remove the
        // anyOf/oneOf since it's just a validation hint.
        let has_properties =
            m.contains_key(&serde_yaml::Value::String("properties".to_string()));
        if has_type && has_properties {
            let mut m = m.clone();
            for key in &["anyOf", "oneOf"] {
                let key_val = serde_yaml::Value::String((*key).to_string());
                if let Some(serde_yaml::Value::Sequence(items)) = m.get(&key_val) {
                    let all_required_only = items.iter().all(|item| {
                        if let serde_yaml::Value::Mapping(im) = item {
                            im.len() == 1
                                && im.contains_key(&serde_yaml::Value::String(
                                    "required".to_string(),
                                ))
                        } else {
                            false
                        }
                    });
                    if all_required_only {
                        m.remove(&key_val);
                    }
                }
            }
            return serde_yaml::Value::Mapping(m);
        }
    }
    value
}

/// Keys that often contain human-readable strings that don't match the
/// declared type, causing strict OpenAPI parsers to fail.
const PROBLEMATIC_KEYS: &[&str] = &[
    "default",
    "example",
    "examples",
    "maximum",
    "minimum",
    "exclusiveMaximum",
    "exclusiveMinimum",
    "maxLength",
    "minLength",
    "maxItems",
    "minItems",
    "pattern",
    "discriminator",
];

/// Recursively strip problematic keys from a YAML value tree.
fn strip_problematic_keys(value: serde_yaml::Value) -> serde_yaml::Value {
    match value {
        serde_yaml::Value::Mapping(mut m) => {
            for key in PROBLEMATIC_KEYS {
                m.remove(&serde_yaml::Value::String((*key).to_string()));
            }
            let cleaned: serde_yaml::Mapping = m
                .into_iter()
                .map(|(k, v)| (k, strip_problematic_keys(v)))
                .collect();
            serde_yaml::Value::Mapping(cleaned)
        }
        serde_yaml::Value::Sequence(seq) => {
            serde_yaml::Value::Sequence(seq.into_iter().map(strip_problematic_keys).collect())
        }
        other => other,
    }
}

fn parse_schema(name: &str, schema: &OASchema, spec: &OpenAPI) -> Option<Schema> {
    let type_name = to_type_name(name);

    match &schema.schema_kind {
        OASchemaKind::Type(ty) => match ty {
            Type::Object(obj) => Some(Schema {
                name: type_name,
                kind: SchemaKind::Struct(parse_object(obj, spec)),
            }),
            Type::String(string_type) => {
                if !string_type.enumeration.is_empty() {
                    Some(Schema {
                        name: type_name,
                        kind: SchemaKind::Enum(parse_string_enum(string_type)),
                    })
                } else {
                    Some(Schema {
                        name: type_name,
                        kind: SchemaKind::StringNewtype,
                    })
                }
            }
            Type::Array(arr) => {
                let inner = resolve_array_item_type(arr, spec);
                Some(Schema {
                    name: type_name,
                    kind: SchemaKind::TypeAlias(format!("Vec<{}>", inner.to_rust_type())),
                })
            }
            Type::Integer(_) => Some(Schema {
                name: type_name,
                kind: SchemaKind::TypeAlias("i64".to_string()),
            }),
            Type::Number(_) => Some(Schema {
                name: type_name,
                kind: SchemaKind::TypeAlias("f64".to_string()),
            }),
            Type::Boolean(_) => Some(Schema {
                name: type_name,
                kind: SchemaKind::TypeAlias("bool".to_string()),
            }),
        },
        OASchemaKind::AllOf { all_of } => {
            let mut fields = Vec::new();
            let mut all_required: Vec<String> = Vec::new();

            for item in all_of {
                match item {
                    ReferenceOr::Item(sub_schema) => {
                        if let OASchemaKind::Type(Type::Object(obj)) = &sub_schema.schema_kind {
                            all_required.extend(obj.required.clone());
                        }
                    }
                    ReferenceOr::Reference { reference } => {
                        if let Some(resolved) = resolve_ref_schema(reference, spec) {
                            collect_required(resolved, spec, &mut all_required);
                        }
                    }
                }
            }

            for item in all_of {
                match item {
                    ReferenceOr::Item(sub_schema) => {
                        if let OASchemaKind::Type(Type::Object(obj)) = &sub_schema.schema_kind {
                            let sub = parse_object_with_required(obj, &all_required, spec);
                            fields.extend(sub.fields);
                        }
                    }
                    ReferenceOr::Reference { reference } => {
                        if let Some(resolved) = resolve_ref_schema(reference, spec) {
                            let merged = collect_all_fields(resolved, spec);
                            for (prop_name, prop_schema) in &merged {
                                let field_type = resolve_field_type_from_ref(prop_schema, spec);
                                let rust_name = crate::naming::to_field_name(prop_name);
                                let is_required = all_required.contains(prop_name);
                                let final_type = if is_required {
                                    field_type
                                } else {
                                    FieldType::Option(Box::new(field_type))
                                };
                                let serde_rename = if rust_name != *prop_name {
                                    Some(prop_name.to_string())
                                } else {
                                    None
                                };
                                fields.push(FieldDef {
                                    name: prop_name.to_string(),
                                    rust_name,
                                    ty: final_type,
                                    required: is_required,
                                    serde_rename,
                                });
                            }
                        }
                    }
                }
            }

            let mut seen = std::collections::HashSet::new();
            fields.retain(|f| seen.insert(f.name.clone()));

            if fields.is_empty() {
                // If allOf has a single $ref and resolved to no fields, emit type alias
                let refs: Vec<&str> = all_of
                    .iter()
                    .filter_map(|item| match item {
                        ReferenceOr::Reference { reference } => Some(reference.as_str()),
                        _ => None,
                    })
                    .collect();
                if refs.len() == 1 {
                    let ref_type = resolve_ref_type(refs[0], spec);
                    Some(Schema {
                        name: type_name,
                        kind: SchemaKind::TypeAlias(ref_type.to_rust_type()),
                    })
                } else {
                    None
                }
            } else {
                Some(Schema {
                    name: type_name,
                    kind: SchemaKind::Struct(StructDef { fields }),
                })
            }
        }
        OASchemaKind::OneOf { one_of } => parse_complex_enum(&type_name, one_of, spec),
        OASchemaKind::AnyOf { any_of } => {
            let mut fields = Vec::new();

            for item in any_of {
                match item {
                    ReferenceOr::Item(sub_schema) => {
                        if let OASchemaKind::Type(Type::Object(obj)) = &sub_schema.schema_kind {
                            let sub = parse_object_all_optional(obj, spec);
                            fields.extend(sub.fields);
                        }
                    }
                    ReferenceOr::Reference { reference } => {
                        if let Some(resolved) = resolve_ref_schema(reference, spec) {
                            let merged = collect_all_fields(resolved, spec);
                            for (prop_name, prop_schema) in &merged {
                                let field_type = resolve_field_type_from_ref(prop_schema, spec);
                                let rust_name = crate::naming::to_field_name(prop_name);
                                let serde_rename = if rust_name != *prop_name {
                                    Some(prop_name.to_string())
                                } else {
                                    None
                                };
                                fields.push(FieldDef {
                                    name: prop_name.to_string(),
                                    rust_name,
                                    ty: FieldType::Option(Box::new(field_type)),
                                    required: false,
                                    serde_rename,
                                });
                            }
                        }
                    }
                }
            }

            let mut seen = std::collections::HashSet::new();
            fields.retain(|f| seen.insert(f.name.clone()));

            if fields.is_empty() {
                None
            } else {
                Some(Schema {
                    name: type_name,
                    kind: SchemaKind::Struct(StructDef { fields }),
                })
            }
        }
        OASchemaKind::Not { .. } => None,
        OASchemaKind::Any(any_schema) => {
            // AnySchema occurs when openapiv3 can't determine the kind (e.g.,
            // when `type: object` coexists with `allOf`). Extract fields from
            // properties if present, or from allOf/oneOf references.

            // Check for bare enum (no type specified)
            if !any_schema.enumeration.is_empty() {
                let values: Vec<String> = any_schema
                    .enumeration
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !values.is_empty() {
                    let rename_all = crate::naming::detect_rename_all(&values);
                    let variants: Vec<EnumVariant> = dedup_variants(
                        values
                            .iter()
                            .map(|v| EnumVariant {
                                name: to_variant_name(v),
                                original: v.clone(),
                            })
                            .collect(),
                    );
                    return Some(Schema {
                        name: type_name,
                        kind: SchemaKind::Enum(EnumDef {
                            variants,
                            rename_all,
                            is_untagged: false,
                            is_complex: false,
                            complex_variants: Vec::new(),
                        }),
                    });
                }
            }

            // Check for object with only additionalProperties (HashMap wrapper)
            if any_schema.properties.is_empty()
                && any_schema.all_of.is_empty()
                && any_schema.one_of.is_empty()
            {
                if let Some(AdditionalProperties::Schema(inner_schema)) =
                    &any_schema.additional_properties
                {
                    let inner = resolve_field_type_from_boxed_ref(inner_schema, spec);
                    return Some(Schema {
                        name: type_name,
                        kind: SchemaKind::TypeAlias(format!(
                            "std::collections::HashMap<String, {}>",
                            inner.to_rust_type()
                        )),
                    });
                }
            }

            let mut fields = Vec::new();

            // Try properties first
            for (prop_name, prop_schema) in &any_schema.properties {
                let field_type = resolve_field_type_from_ref(prop_schema, spec);
                let rust_name = crate::naming::to_field_name(prop_name);
                let is_required = any_schema.required.contains(prop_name);
                let final_type = if is_required {
                    field_type
                } else {
                    FieldType::Option(Box::new(field_type))
                };
                let serde_rename = if rust_name != *prop_name {
                    Some(prop_name.to_string())
                } else {
                    None
                };
                fields.push(FieldDef {
                    name: prop_name.to_string(),
                    rust_name,
                    ty: final_type,
                    required: is_required,
                    serde_rename,
                });
            }

            // If no properties, try allOf refs
            if fields.is_empty() {
                for item in &any_schema.all_of {
                    if let ReferenceOr::Reference { reference } = item {
                        if let Some(resolved) = resolve_ref_schema(reference, spec) {
                            let merged = collect_all_fields(resolved, spec);
                            let mut all_required = Vec::new();
                            collect_required(resolved, spec, &mut all_required);
                            for (prop_name, prop_schema) in &merged {
                                let field_type = resolve_field_type_from_ref(prop_schema, spec);
                                let rust_name = crate::naming::to_field_name(prop_name);
                                let is_required = all_required.contains(prop_name);
                                let final_type = if is_required {
                                    field_type
                                } else {
                                    FieldType::Option(Box::new(field_type))
                                };
                                let serde_rename = if rust_name != *prop_name {
                                    Some(prop_name.to_string())
                                } else {
                                    None
                                };
                                fields.push(FieldDef {
                                    name: prop_name.to_string(),
                                    rust_name,
                                    ty: final_type,
                                    required: is_required,
                                    serde_rename,
                                });
                            }
                        }
                    }
                }
            }

            // If still no fields, try oneOf for enum generation
            if fields.is_empty() && !any_schema.one_of.is_empty() {
                return parse_complex_enum(&type_name, &any_schema.one_of, spec);
            }

            if fields.is_empty() {
                // Last resort: if there are allOf refs, emit a type alias to the first
                if any_schema.all_of.len() == 1 {
                    if let ReferenceOr::Reference { reference } = &any_schema.all_of[0] {
                        let ref_type = resolve_ref_type(reference, spec);
                        return Some(Schema {
                            name: type_name,
                            kind: SchemaKind::TypeAlias(ref_type.to_rust_type()),
                        });
                    }
                }
                None
            } else {
                let mut seen = std::collections::HashSet::new();
                fields.retain(|f| seen.insert(f.name.clone()));
                Some(Schema {
                    name: type_name,
                    kind: SchemaKind::Struct(StructDef { fields }),
                })
            }
        }
    }
}

fn collect_required(schema: &OASchema, spec: &OpenAPI, required: &mut Vec<String>) {
    match &schema.schema_kind {
        OASchemaKind::Type(Type::Object(obj)) => {
            required.extend(obj.required.clone());
        }
        OASchemaKind::AllOf { all_of } => {
            for item in all_of {
                match item {
                    ReferenceOr::Item(sub) => {
                        collect_required(sub, spec, required);
                    }
                    ReferenceOr::Reference { reference } => {
                        if let Some(resolved) = resolve_ref_schema(reference, spec) {
                            collect_required(resolved, spec, required);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn collect_all_fields(
    schema: &OASchema,
    spec: &OpenAPI,
) -> Vec<(String, ReferenceOr<Box<OASchema>>)> {
    let mut result = Vec::new();

    match &schema.schema_kind {
        OASchemaKind::Type(Type::Object(obj)) => {
            for (name, prop) in &obj.properties {
                result.push((name.clone(), prop.clone()));
            }
        }
        OASchemaKind::AllOf { all_of } => {
            for item in all_of {
                match item {
                    ReferenceOr::Item(sub) => {
                        result.extend(collect_all_fields(sub, spec));
                    }
                    ReferenceOr::Reference { reference } => {
                        if let Some(resolved) = resolve_ref_schema(reference, spec) {
                            result.extend(collect_all_fields(resolved, spec));
                        }
                    }
                }
            }
        }
        _ => {}
    }

    result
}

fn parse_object(obj: &ObjectType, spec: &OpenAPI) -> StructDef {
    parse_object_with_required(obj, &obj.required, spec)
}

fn parse_object_with_required(
    obj: &ObjectType,
    required: &[String],
    spec: &OpenAPI,
) -> StructDef {
    let mut fields = Vec::new();

    for (prop_name, prop_schema) in &obj.properties {
        let field_type = resolve_field_type_from_ref(prop_schema, spec);
        let rust_name = crate::naming::to_field_name(prop_name);
        let is_required = required.contains(prop_name);

        let final_type = if is_required {
            field_type
        } else {
            FieldType::Option(Box::new(field_type))
        };

        let serde_rename = if rust_name != *prop_name {
            Some(prop_name.to_string())
        } else {
            None
        };

        fields.push(FieldDef {
            name: prop_name.to_string(),
            rust_name,
            ty: final_type,
            required: is_required,
            serde_rename,
        });
    }

    if let Some(AdditionalProperties::Schema(schema_ref)) = &obj.additional_properties {
        if fields.is_empty() {
            let inner_type = resolve_field_type_from_boxed_ref(schema_ref, spec);
            fields.push(FieldDef {
                name: "_additional".to_string(),
                rust_name: "_additional".to_string(),
                ty: FieldType::HashMap(Box::new(inner_type)),
                required: true,
                serde_rename: None,
            });
        }
    }

    StructDef { fields }
}

fn parse_object_all_optional(obj: &ObjectType, spec: &OpenAPI) -> StructDef {
    let mut fields = Vec::new();

    for (prop_name, prop_schema) in &obj.properties {
        let field_type = resolve_field_type_from_ref(prop_schema, spec);
        let rust_name = crate::naming::to_field_name(prop_name);

        let serde_rename = if rust_name != *prop_name {
            Some(prop_name.to_string())
        } else {
            None
        };

        fields.push(FieldDef {
            name: prop_name.to_string(),
            rust_name,
            ty: FieldType::Option(Box::new(field_type)),
            required: false,
            serde_rename,
        });
    }

    StructDef { fields }
}

fn parse_string_enum(string_type: &StringType) -> EnumDef {
    let values: Vec<String> = string_type
        .enumeration
        .iter()
        .filter_map(|v| v.clone())
        .collect();

    let rename_all = crate::naming::detect_rename_all(&values);

    let variants: Vec<EnumVariant> = dedup_variants(
        values
            .iter()
            .map(|v| {
                let variant_name = to_variant_name(v);
                EnumVariant {
                    name: variant_name,
                    original: v.clone(),
                }
            })
            .collect(),
    );

    EnumDef {
        variants,
        rename_all,
        is_untagged: false,
        is_complex: false,
        complex_variants: Vec::new(),
    }
}

/// Deduplicate enum variants by their Rust name, keeping the first occurrence.
fn dedup_variants(variants: Vec<EnumVariant>) -> Vec<EnumVariant> {
    let mut seen = std::collections::HashSet::new();
    variants
        .into_iter()
        .filter(|v| seen.insert(v.name.clone()))
        .collect()
}

fn parse_complex_enum(
    type_name: &str,
    variants: &[ReferenceOr<OASchema>],
    _spec: &OpenAPI,
) -> Option<Schema> {
    let mut complex_variants = Vec::new();

    for variant in variants {
        match variant {
            ReferenceOr::Reference { reference } => {
                let ref_name = ref_to_type_name(reference);
                complex_variants.push(ComplexVariant {
                    name: to_type_name(&ref_name),
                    ty: FieldType::Ref(to_type_name(&ref_name)),
                });
            }
            ReferenceOr::Item(schema) => {
                if let OASchemaKind::Type(Type::Object(_)) = &schema.schema_kind {
                    let title = schema
                        .schema_data
                        .title
                        .clone()
                        .unwrap_or_else(|| format!("Variant{}", complex_variants.len()));
                    complex_variants.push(ComplexVariant {
                        name: to_type_name(&title),
                        ty: FieldType::Ref(to_type_name(&title)),
                    });
                }
            }
        }
    }

    if complex_variants.is_empty() {
        return None;
    }

    Some(Schema {
        name: type_name.to_string(),
        kind: SchemaKind::Enum(EnumDef {
            variants: Vec::new(),
            rename_all: None,
            is_untagged: true,
            is_complex: true,
            complex_variants,
        }),
    })
}

fn resolve_field_type_from_ref(
    schema_ref: &ReferenceOr<Box<OASchema>>,
    spec: &OpenAPI,
) -> FieldType {
    match schema_ref {
        ReferenceOr::Reference { reference } => resolve_ref_type(reference, spec),
        ReferenceOr::Item(schema) => resolve_field_type(schema, spec),
    }
}

fn resolve_field_type_from_boxed_ref(
    schema_ref: &Box<ReferenceOr<OASchema>>,
    spec: &OpenAPI,
) -> FieldType {
    match schema_ref.as_ref() {
        ReferenceOr::Reference { reference } => resolve_ref_type(reference, spec),
        ReferenceOr::Item(schema) => resolve_field_type(schema, spec),
    }
}

fn resolve_ref_type(reference: &str, spec: &OpenAPI) -> FieldType {
    let ref_name = ref_to_type_name(reference);
    let type_name = to_type_name(&ref_name);

    if let Some(resolved) = resolve_ref_schema(reference, spec) {
        if let OASchemaKind::Type(ty) = &resolved.schema_kind {
            match ty {
                Type::String(st) => {
                    if st.enumeration.is_empty() {
                        return FieldType::String;
                    }
                }
                Type::Array(arr) => {
                    let inner = resolve_array_item_type(arr, spec);
                    return FieldType::Vec(Box::new(inner));
                }
                Type::Integer(_) => return FieldType::I64,
                Type::Number(_) => return FieldType::F64,
                Type::Boolean(_) => return FieldType::Bool,
                _ => {}
            }
        }
    }

    FieldType::Ref(type_name)
}

fn resolve_field_type(schema: &OASchema, spec: &OpenAPI) -> FieldType {
    match &schema.schema_kind {
        OASchemaKind::Type(ty) => match ty {
            Type::String(_) => FieldType::String,
            Type::Number(_) => FieldType::F64,
            Type::Integer(int_type) => {
                if int_type.minimum.is_some_and(|m| m >= 0) {
                    FieldType::U64
                } else {
                    FieldType::I64
                }
            }
            Type::Boolean(_) => FieldType::Bool,
            Type::Array(arr) => {
                let inner = resolve_array_item_type(arr, spec);
                FieldType::Vec(Box::new(inner))
            }
            Type::Object(obj) => {
                if let Some(AdditionalProperties::Schema(inner_schema)) =
                    &obj.additional_properties
                {
                    let inner = resolve_field_type_from_boxed_ref(inner_schema, spec);
                    FieldType::HashMap(Box::new(inner))
                } else {
                    FieldType::Ref("serde_json::Value".to_string())
                }
            }
        },
        OASchemaKind::AllOf { .. }
        | OASchemaKind::OneOf { .. }
        | OASchemaKind::AnyOf { .. } => FieldType::Ref("serde_json::Value".to_string()),
        OASchemaKind::Not { .. } | OASchemaKind::Any(_) => {
            FieldType::Ref("serde_json::Value".to_string())
        }
    }
}

fn resolve_array_item_type(arr: &ArrayType, spec: &OpenAPI) -> FieldType {
    match &arr.items {
        Some(items) => resolve_field_type_from_ref(items, spec),
        None => FieldType::Ref("serde_json::Value".to_string()),
    }
}

fn resolve_ref_schema<'a>(reference: &str, spec: &'a OpenAPI) -> Option<&'a OASchema> {
    let name = ref_to_type_name(reference);
    spec.components.as_ref()?.schemas.get(&name).and_then(|s| {
        if let ReferenceOr::Item(schema) = s {
            Some(schema)
        } else {
            None
        }
    })
}

fn ref_to_type_name(reference: &str) -> String {
    reference
        .rsplit('/')
        .next()
        .unwrap_or(reference)
        .to_string()
}
