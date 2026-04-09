use crate::cli::CategoryCommands;
use crate::output::{format_bool, print_json, print_table, OutputFormat};
use anyhow::Result;
use mmex_lib::domain::categories::{Category, CategoryUpdate};
use mmex_lib::domain::types::CategoryId;
use mmex_lib::MmexContext;
use serde_json::json;

pub fn execute(ctx: &MmexContext, cmd: &CategoryCommands, format: OutputFormat) -> Result<()> {
    match cmd {
        CategoryCommands::List => list(ctx, format),
        CategoryCommands::Get { id } => get(ctx, *id, format),
        CategoryCommands::Subcategories { parent_id } => subcategories(ctx, *parent_id, format),
        CategoryCommands::Create { name, parent_id } => create(ctx, name, *parent_id, format),
        CategoryCommands::Update {
            id,
            name,
            active,
            parent_id,
        } => update(ctx, *id, name, *active, *parent_id, format),
        CategoryCommands::UpdatePartial {
            id,
            name,
            active,
            parent_id,
        } => update_partial(ctx, *id, name.clone(), *active, *parent_id, format),
        CategoryCommands::Delete { id } => delete(ctx, *id, format),
    }
}

fn list(ctx: &MmexContext, format: OutputFormat) -> Result<()> {
    let categories = ctx.categories().get_all_categories()?;

    match format {
        OutputFormat::Json => print_json(&categories),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Active", "Parent ID"];
            let rows: Vec<Vec<String>> = categories
                .iter()
                .map(|c| {
                    vec![
                        c.id.v1.to_string(),
                        c.name.clone(),
                        format_bool(c.active).to_string(),
                        c.parent_id.map(|p| p.v1.to_string()).unwrap_or_default(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn get(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    let category = ctx
        .categories()
        .get_category_by_id(mmex_lib::domain::types::CategoryId::new(id))?;

    match category {
        Some(c) => match format {
            OutputFormat::Json => print_json(&c),
            OutputFormat::Table => {
                let headers = ["Field", "Value"];
                let rows = vec![
                    vec!["ID".to_string(), c.id.v1.to_string()],
                    vec!["Name".to_string(), c.name.clone()],
                    vec!["Active".to_string(), format_bool(c.active).to_string()],
                    vec![
                        "Parent ID".to_string(),
                        c.parent_id.map(|p| p.v1.to_string()).unwrap_or_default(),
                    ],
                ];
                print_table(&headers, &rows);
            }
        },
        None => println!("Category not found"),
    }
    Ok(())
}

fn subcategories(ctx: &MmexContext, parent_id: i64, format: OutputFormat) -> Result<()> {
    let categories = ctx
        .categories()
        .get_subcategories(mmex_lib::domain::types::CategoryId::new(parent_id))?;

    match format {
        OutputFormat::Json => print_json(&categories),
        OutputFormat::Table => {
            let headers = ["ID", "Name", "Active"];
            let rows: Vec<Vec<String>> = categories
                .iter()
                .map(|c| {
                    vec![
                        c.id.v1.to_string(),
                        c.name.clone(),
                        format_bool(c.active).to_string(),
                    ]
                })
                .collect();
            print_table(&headers, &rows);
        }
    }
    Ok(())
}

fn create(
    ctx: &MmexContext,
    name: &str,
    parent_id: Option<i64>,
    format: OutputFormat,
) -> Result<()> {
    let parent = parent_id.map(CategoryId::new);
    let created = ctx.categories().create_category(name, parent)?;

    match format {
        OutputFormat::Json => print_json(&created),
        OutputFormat::Table => {
            println!("Category created successfully with ID {}", created.id.v1);
        }
    }
    Ok(())
}

fn update(
    ctx: &MmexContext,
    id: i64,
    name: &str,
    active: bool,
    parent_id: Option<i64>,
    format: OutputFormat,
) -> Result<()> {
    let category = Category {
        id: CategoryId::new(id),
        name: name.to_string(),
        active,
        parent_id: parent_id.map(CategoryId::new),
    };

    ctx.categories().update_category(&category)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Category {} updated successfully", id);
        }
    }
    Ok(())
}

fn update_partial(
    ctx: &MmexContext,
    id: i64,
    name: Option<String>,
    active: Option<bool>,
    parent_id: Option<i64>,
    format: OutputFormat,
) -> Result<()> {
    let update = CategoryUpdate {
        name,
        active,
        parent_id: parent_id.map(CategoryId::new),
    };

    ctx.categories()
        .update_category_partial(CategoryId::new(id), update)?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Category {} partially updated successfully", id);
        }
    }
    Ok(())
}

fn delete(ctx: &MmexContext, id: i64, format: OutputFormat) -> Result<()> {
    ctx.categories().delete_category(CategoryId::new(id))?;

    match format {
        OutputFormat::Json => print_json(&json!({ "status": "success", "id": id })),
        OutputFormat::Table => {
            println!("Category {} deleted successfully", id);
        }
    }
    Ok(())
}
