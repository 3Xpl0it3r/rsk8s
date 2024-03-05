use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::app::state::Route;
use crate::app::{
    state::{AppState, Mode},
    ui::{
        theme::{self, Kanagawa},
        util::{self as uiutil, debug_widget, horizontal_chunks},
    },
};

pub fn draw_page_pod_list(f: &mut Frame, area: Rect, state: &mut AppState) {
    // split windows chunks[0] for input, chunk[1] for podlist
    let area = uiutil::vertical_chunks(
        vec![
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(3),
        ],
        area,
    );
    let input_area = area[0];
    let ns_select_area = area[1];
    let pod_list_area = area[2];

    // draw user input
    draw_user_input(f, input_area, state);

    draw_namespaces(f, ns_select_area, state);

    draw_pods(f, pod_list_area, state);
    /* let input = state.input_char.to_string(); */

    /* let listitems = uiutil::selectable_list(&state.cache_items); */
}

fn draw_user_input(f: &mut Frame, area: Rect, state: &AppState) {
    let input_widget = if let Route::PodList = state.get_route() {
        uiutil::user_input(state.user_input.as_str(), state.get_mode())
    } else {
        uiutil::user_input("", Mode::Normal)
    };
    f.render_widget(input_widget, area);
}

fn draw_namespaces(f: &mut Frame, area: Rect, state: &AppState) {
    let area = horizontal_chunks(
        vec![Constraint::Percentage(70), Constraint::Percentage(30)],
        area,
    );

    let list = uiutil::selectable_list_1(&state.namespace_cache);

    let help_message = r#"[n]     trigger
[k]     up 
[j]     down
[enter] comfirm
[esc]   quit
"#;

    f.render_widget(list, area[0]);
    f.render_widget(debug_widget(help_message), area[1]);
}

fn draw_pods(f: &mut Frame, area: Rect, state: &AppState) {
    let list = pod_select_items(state);
    f.render_widget(list, area);
}

fn pod_select_items(app_state: &AppState) -> List<'_> {
    let mut list_items = Vec::new();
    let namespace = app_state.namespace_cache.get().unwrap();
    let items = app_state.cache_items.list().iter();
    let title = format!(
        "{:<48}{:<16}{:<16}",
        "Pod".to_string(),
        "Status".to_string(),
        "Ready".to_string()
    );
    list_items.push(ListItem::new(title).style(Style::default()));
    for (idx, val) in items.enumerate() {
        let pod_desc = app_state.pod_describes.get(namespace.as_ref(), val);
        let item_txt: String;
        if let Some(describe) = pod_desc {
            unsafe {
                item_txt = format!(
                    "{:<48}{:<16}{:<16}",
                    val,
                    &(*describe.status),
                    format!("{}/{}", describe.ready_number, describe.containers.len())
                );
            }
        } else {
            item_txt = format!(
                "{:<48}{:<16}{:<16}",
                val,
                "none".to_string(),
                "none".to_string()
            );
        }

        if idx == app_state.cache_items.index() {
            list_items.push(
                ListItem::new(item_txt).style(
                    ratatui::style::Style::default()
                        .fg(theme::DefaultTheme::BLUE_LIGHT)
                        .bg(theme::DefaultTheme::SUMLINK1),
                ),
            );
        } else {
            list_items.push(ListItem::new(item_txt).style(Style::default()));
        }
    }
    List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
}
