use std::borrow::Borrow;

use bevy::{ecs::system::EntityCommands, prelude::*};
use common::{
    flex_container::FlexContainer, screens::MainScreenViewData, states::NavigationState,
    text::TEXT_COLOR, utilities::despawn_screen,
};
use epub::{chapters::chapter_node::ChapterNode, epub::EBook, reader::EBookReader};
use library::library::UserLibrary;

use crate::{
    bundles::{ChapterNodeComponent, HeadingComponentBundle, ParagraphComponentBundle},
    toolbar::ReaderToolbarBundle,
};

#[derive(Component)]
pub struct OnReaderScreen;

pub struct ReaderPlugin;

impl Plugin for ReaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NavigationState::Reader), (reader_setup).chain())
            // .add_systems(Update, ().chain().run_if(in_state(NavigationState::Reader)))
            // .add_systems(Update, ().run_if(in_state(NavigationState::Library)))
            .add_systems(
                OnExit(NavigationState::Reader),
                despawn_screen::<OnReaderScreen>,
            );
    }
}

fn reader_setup(
    mut commands: Commands,
    main_screen_view_data: Res<MainScreenViewData>,
    user_library: Res<UserLibrary>,
) {
    let selected_book = user_library.selected_for_reading().clone();

    let reader_screen = commands
        .spawn((FlexContainer::new(None), OnReaderScreen))
        .with_children(|parent| {
            let toolbar_entity = ReaderToolbarBundle::spawn(parent);
            let mut chapter_content_tntity = commands
                .spawn((FlexContainer::new(None), OnReaderScreen))
                .id();

            if let Some(book) = selected_book {
                let ebook = match EBook::read_epub(book.path.to_string()) {
                    Ok(ebook) => {
                        println!("SUCCESS");
                        println!("{:?}", ebook.table_of_contents);
                        Some(ebook)
                    }
                    Err(e) => {
                        error!("Error reading ebook: {:?}", e);
                        None
                    }
                };

                let reader = EBookReader::new(ebook.unwrap());
                let chapter = reader.current_chapter();
                let body_node = chapter.get_body();

                if let Some(body_node) = body_node {
                    chapter_content_tntity = create_html_nodes(body_node.borrow(), commands);
                }
            }
        })
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[reader_screen]);
}

//TODO: Change this approach to only preparing boundles that need to be spawned instead of spawning entities
fn create_html_nodes(body_node: &ChapterNode, commands: &mut ChildBuilder) -> Entity {
    let mut entity = commands.spawn((FlexContainer::new(None), OnReaderScreen));

    for child in body_node.get_children().iter() {
        let child_entity: EntityCommands = match child.tag.as_str() {
            _ => commands.spawn(TextBundle::from_section(
                child.content.borrow().clone(),
                TextStyle {
                    font_size: 20.0,
                    color: TEXT_COLOR,
                    ..default()
                },
            )),
        };

        // let child_entity = create_html_nodes_for_children(&child, commands, child_entity);

        entity.push_children(&[child_entity.id()]);
    }

    entity.id()
}

struct ChapterNodeBundle {
    bundle: ChapterNodeComponent,
    children: Vec<ChapterNodeComponent>,
}

impl ChapterNodeBundle {
    fn from_chapter_node(chapter_node: &ChapterNode) -> ChapterNodeComponent {
        let mut children = vec![];

        for child in chapter_node.get_children().iter() {
            let child_bundle = ChapterNodeBundle::from_chapter_node(child.borrow());
            children.push(child_bundle);
        }

        let node: ChapterNodeComponent = match chapter_node.tag.as_str() {
            "div" => ChapterNodeComponent::Heading(HeadingComponentBundle::new(
                chapter_node.content.borrow().as_str(),
                children,
            )),
            "h1" => ChapterNodeComponent::Heading(HeadingComponentBundle::new(
                chapter_node.content.borrow().as_str(),
                children,
            )),
            _ => ChapterNodeComponent::Paragraph(ParagraphComponentBundle::new(
                chapter_node.content.borrow().as_str(),
                children,
            )),
        };

        node
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::storage::Table;
    use epub::{
        chapters::chapter::Chapter, table_of_contents::table_of_contents_item::TableOfContentsItem,
    };

    use super::*;

    #[test]
    fn should_create_chapter_node_bundles_from_equivalent_chapter_nodes() {
        //arrange
        let chapter_content: &str = r#"
            <h1>Chapter 1</h1>
            <div>Hello there - <i>said Obi Wan</i></div>
            <p>'General Kenobi' - replied Grievous</p>
            <div>
                <div>1</div>
                <div>2</div>
                <div>3
                    <div>3a</div>
                    <div>3b</div>
                </div>
            </div>
        "#;

        let toc_item = TableOfContentsItem::new(
            String::new(),
            String::new(),
            Some(chapter_content.to_string()),
        );

        //act
        let chapter_node = Chapter::from_item_with_content(toc_item, chapter_content.to_string());
        let sut = ChapterNodeBundle::from_chapter_node(chapter_node.recreated_structure.borrow());

        //assert
        // assert_eq!(sut.borrow().len(), 4);
        // assert_eq!(sut.bundle.node., "h1");
    }
}

// fn create_html_nodes_for_children(
//     current_node: &ChapterNode,
//     mut commands: Commands,
//     parent_entity_commands: EntityCommands,
// ) -> Entity {
//     for child in current_node.get_children().iter() {
//         let child_entity: EntityCommands = match child.tag.as_str() {
//             _ => commands.spawn(TextBundle::from_section(
//                 child.content.borrow().clone(),
//                 TextStyle {
//                     font_size: 20.0,
//                     color: TEXT_COLOR,
//                     ..default()
//                 },
//             )),
//         };

//         let child_entity = create_html_nodes_for_children(&child, commands, child_entity);
//         parent_entity_commands.push_children(&[child_entity]);
//     }

//     parent_entity_commands.id()
// }
