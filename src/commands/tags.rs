use crate::cli::TagCommands;
use crate::output::{print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::tags::{Tag, TagUpdate};
use mmex_lib::domain::types::TagId;
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &TagCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        TagCommands::List => list(ctx, format),
        TagCommands::Get { id } => get(ctx, *id, format),
        TagCommands::Create { name } => create(ctx, name, format),
        TagCommands::Update { id, name } => update(ctx, *id, name, format),
        TagCommands::UpdatePartial { id, name } => update_partial(ctx, *id, name.clone(), format),
        TagCommands::Delete { id } => delete(ctx, *id, format),
        TagCommands::GetForReference { ref_type, ref_id } => {
            get_for_reference(ctx, ref_type, *ref_id, format)
        }
        TagCommands::LinkToReference {
            ref_type,
            ref_id,
            tag_id,
        } => link_to_reference(ctx, ref_type, *ref_id, *tag_id, format),
        TagCommands::UnlinkFromReference {
            ref_type,
            ref_id,
            tag_id,
        } => unlink_from_reference(ctx, ref_type, *ref_id, *tag_id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let tags = ctx.tags().get_all_tags()?;

    match format {
        OutputFormat::Json => print_json(&tags),
        OutputFormat::Table => {
            let headers = ["ID", "Name"];
            let rows: Vec<Vec<String>> = tags
                .iter()
                .map(|t| vec![t.id.v1.to_string(), t.name.clone()])
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let tag = ctx
        .tags()
        .get_tag_by_id(mmex_lib::domain::types::TagId::new(id))?;

    match tag {
        Some(t) => match format {
            OutputFormat::Json => print_json(&t),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), t.id.v1.to_string()],
                    vec!["Name".to_string(), t.name.clone()],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Tag not found"),
    }
    Ok(())
}

fn create(ctx: &MmexContext, name: &str, format: OutputFormat) -> Result<()> {
    let created = ctx.tags().create_tag(name)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Tag created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(ctx: &MmexContext, id: i64, name: &str, format: OutputFormat) -> Result<()> {
    let tag = Tag {
        id: TagId::new(id),
        name: name.to_string(),
    };

    ctx.tags().update_tag(&tag)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Tag {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    name: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let update = TagUpdate { name };

    ctx.tags().update_tag_partial(TagId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Tag {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.tags().delete_tag(TagId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Tag {} deleted successfully", id);
        }
    }
    Ok(())
}

fn get_for_reference(
    ctx: &MmexContext,
    ref_type: &str,
    ref_id: i64,
    format: OutputFormat,
) -> Result<()> {
    let tags = ctx.tags().get_for_reference(ref_type, ref_id)?;

    match format {
        OutputFormat::Json => print_json(&tags),
        OutputFormat::Table => {
            let headers = ["ID", "Name"];
            let rows: Vec<Vec<String>> = tags
                .iter()
                .map(|t| vec![t.id.v1.to_string(), t.name.clone()])
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn link_to_reference(
    ctx: &MmexContext,
    ref_type: &str,
    ref_id: i64,
    tag_id: i64,
    format: OutputFormat,
) -> Result<()> {
    ctx.tags()
        .link_to_reference(ref_type, ref_id, TagId::new(tag_id))?;

    match format {
        OutputFormat::Json => print_json(
            &json!({ "status": "success", "tag_id": tag_id, "ref_type": ref_type, "ref_id": ref_id }),
        ),
        OutputFormat::Table => {
            println!(
                "Tag {} linked to {} {} successfully",
                tag_id, ref_type, ref_id
            );
        }
    }
    Ok(())
}

fn unlink_from_reference(
    ctx: &MmexContext,
    ref_type: &str,
    ref_id: i64,
    tag_id: i64,
    format: OutputFormat,
) -> Result<()> {
    ctx.tags()
        .unlink_from_reference(ref_type, ref_id, TagId::new(tag_id))?;

    match format {
        OutputFormat::Json => print_json(
            &json!({ "status": "success", "tag_id": tag_id, "ref_type": ref_type, "ref_id": ref_id }),
        ),
        OutputFormat::Table => {
            println!(
                "Tag {} unlinked from {} {} successfully",
                tag_id, ref_type, ref_id
            );
        }
    }
    Ok(())
}
