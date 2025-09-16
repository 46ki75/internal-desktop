use futures::TryStreamExt;
use notionrs::PaginateExt;
use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkEntity {
    pub id: String,
    pub name: Option<String>,
    pub url: Option<String>,
    pub favicon: Option<String>,
    pub tag: Option<BookmarkTagEntity>,
    pub nsfw: bool,
    pub favorite: bool,
    pub notion_url: String,
}

impl TryFrom<PageResponse> for BookmarkEntity {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: PageResponse) -> Result<Self, Self::Error> {
        let id = value.id;

        let properties = value.properties;

        let name = properties.get("Name").unwrap();

        let url = properties.get("URL").unwrap();

        let tag = properties.get("Tag").unwrap();

        let nsfw = properties.get("NSFW").unwrap();

        let favorite = properties.get("Favorite").unwrap();

        let select = if let PageProperty::Select(select) = tag {
            select.clone().select.and_then(|select| {
                Some(BookmarkTagEntity {
                    id: select.id.unwrap(),
                    name: select.name,
                    color: select
                        .color
                        .map(|color| {
                            match color {
                                SelectColor::Default => "#868e9c",
                                SelectColor::Blue => "#6987b8",
                                SelectColor::Brown => "#a17c5b",
                                SelectColor::Gray => "#59b57c",
                                SelectColor::Green => "#59b57c",
                                SelectColor::Orange => "#d48b70",
                                SelectColor::Pink => "#c9699e",
                                SelectColor::Purple => "#9771bd",
                                SelectColor::Red => "#c56565",
                                SelectColor::Yellow => "#cdb57b",
                            }
                            .to_string()
                        })
                        .unwrap(),
                })
            })
        } else {
            None
        };

        Ok(BookmarkEntity {
            id,
            name: Some(name.to_string()),
            url: Some(url.to_string()),
            favicon: value.icon.map(|f| f.to_string()),
            tag: select,
            nsfw: match nsfw {
                PageProperty::Checkbox(page_checkbox_property) => page_checkbox_property.checkbox,
                _ => true,
            },
            favorite: match favorite {
                PageProperty::Checkbox(page_checkbox_property) => page_checkbox_property.checkbox,
                _ => false,
            },
            notion_url: value.url,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkTagEntity {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[tauri::command]
pub async fn fetch_bookmark_list(app: AppHandle) -> Result<Vec<BookmarkEntity>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let data_source_id = store
        .get("notionBookmarkDataSourceId")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let notion_api_key = store
        .get("notionApiKey")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let notionrs_client = notionrs::Client::new(notion_api_key);

    let response: Vec<PageResponse> = notionrs_client
        .query_data_source()
        .data_source_id(data_source_id)
        .into_stream()
        .try_collect()
        .await
        .unwrap();

    let bookmark = response
        .into_iter()
        .map(|page| BookmarkEntity::try_from(page).unwrap())
        .collect::<Vec<BookmarkEntity>>();

    Ok(bookmark)
}
