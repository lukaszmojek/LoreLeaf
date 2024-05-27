use std::borrow::Borrow;

use bevy::{ecs::system::EntityCommands, prelude::*};
use common::{
    flex_container::{FlexContainer, FlexContainerStyle},
    screens::MainScreenViewData,
    states::NavigationState,
    text::TEXT_COLOR,
    utilities::despawn_screen,
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

            let chapter_content_style = FlexContainerStyle {
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                align_content: AlignContent::SpaceEvenly,
                justify_content: JustifyContent::SpaceEvenly,
                width: Val::Percent(100.0),
                height: Val::Auto,
                overflow: Overflow {
                    x: OverflowAxis::Clip,
                    y: OverflowAxis::Clip,
                },
                background_color: BackgroundColor::from(Color::rgba_from_array([
                    1.0, 0.2, 1.0, 0.5,
                ])),
                ..default()
            };
            let mut chapter_content_entity = parent.spawn((
                FlexContainer::new(Some(chapter_content_style)),
                OnReaderScreen,
            ));

            if let Some(book) = selected_book {
                let ebook = match EBook::read_epub(book.path.to_string()) {
                    Ok(ebook) => {
                        // println!("SUCCESS");
                        // println!("{:?}", ebook.table_of_contents);
                        Some(ebook)
                    }
                    Err(e) => {
                        error!("Error reading ebook: {:?}", e);
                        None
                    }
                };

                let mut reader = EBookReader::new(ebook.unwrap());
                //TODO: Those are temporary, just to test how content is displayed
                reader.move_to_next_chapter();
                reader.move_to_next_chapter();
                reader.move_to_next_chapter();
                reader.move_to_next_chapter();
                reader.move_to_next_chapter();

                let chapter = reader.current_chapter();

                if let Some(body_node) = chapter.get_body() {
                    chapter_content_entity.with_children(move |content_container_node| {
                        let chapter_content_nodes =
                            create_chapter_content_nodes(body_node.borrow());

                        content_container_node.spawn(TextBundle::from_section(
                            "ratatatattatat",
                            TextStyle {
                                font_size: 24.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        ));

                        println!("{:?}", chapter_content_nodes);

                        for node in chapter_content_nodes.into_iter().take(20) {
                            let node_to_spawn = match node {
                                ChapterNodeComponent::Paragraph(bundle) => bundle.node,
                                ChapterNodeComponent::Heading(bundle) => bundle.node,
                                _ => panic!("Unexpected enum variant"),
                            };

                            content_container_node.spawn(node_to_spawn);
                        }
                    });
                }

                println!("HERE");
            }
        })
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[reader_screen]);
}

fn create_chapter_content_nodes(chapter_node: &ChapterNode) -> Vec<ChapterNodeComponent> {
    let mut chapter_nodes = vec![];

    for child in chapter_node.get_children().iter() {
        let chapter_node = map_to_chapter_node_component(child.borrow());
        chapter_nodes.push(chapter_node);
    }

    chapter_nodes
}

fn map_to_chapter_node_component(chapter_node: &ChapterNode) -> ChapterNodeComponent {
    let node: ChapterNodeComponent = match chapter_node.tag.as_str() {
        "div" => ChapterNodeComponent::Paragraph(ParagraphComponentBundle::new(
            chapter_node.content.borrow().as_str(),
        )),
        "p" => ChapterNodeComponent::Paragraph(ParagraphComponentBundle::new(
            chapter_node.content.borrow().as_str(),
        )),
        "h1" => ChapterNodeComponent::Heading(HeadingComponentBundle::new(
            chapter_node.content.borrow().as_str(),
        )),
        _ => ChapterNodeComponent::Paragraph(ParagraphComponentBundle::new(
            chapter_node.content.borrow().as_str(),
        )),
    };

    node
}

#[cfg(test)]
mod tests {
    use epub::{
        chapters::chapter::Chapter, table_of_contents::table_of_contents_item::TableOfContentsItem,
    };

    use super::*;

    #[test]
    fn should_create_chapter_node_bundles_from_equivalent_chapter_nodes() {
        //arrange
        let chapter_content: &str = r#"
            <head>
                <title>Chapter 1</title>
            </head>
            <body>
                <section class="meeting">
                    <h1>Meeting with the council</h1>
                    <p>'Hello there' - said Obi Wan Kenobi</p>
                    <p>'General Kenobi' - replied Grievous</p>
                </section>
            </body>
        "#;

        let toc_item = TableOfContentsItem::new(
            String::new(),
            String::new(),
            Some(chapter_content.to_string()),
        );

        //act
        let chapter_node = Chapter::from_item_with_content(toc_item, chapter_content.to_string());
        let body_node = chapter_node.get_body().expect("Body node not found");
        let sut = create_chapter_content_nodes(body_node.borrow());

        //assert
        assert_eq!(sut.len(), 3);

        match &sut[0] {
            ChapterNodeComponent::Heading(bundle) => {}
            _ => panic!("Unexpected enum variant"),
        }

        match &sut[1] {
            ChapterNodeComponent::Paragraph(bundle) => {}
            _ => panic!("Unexpected enum variant"),
        }

        match &sut[2] {
            ChapterNodeComponent::Paragraph(bundle) => {}
            _ => panic!("Unexpected enum variant"),
        }
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
