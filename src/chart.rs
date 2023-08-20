use charming::{
    component::{Axis, Title, VisualMap, VisualMapType},
    element::{TextStyle, Tooltip, Trigger},
    series::Line,
    theme::Theme,
    Chart, HtmlRenderer,
};

pub fn render(data: Vec<u32>) {
    let mut data_list = vec![];
    let mut value_list = vec![];

    let mut counter = 0;
    data.into_iter().for_each(|value| {
        counter += 1;
        data_list.push(counter.to_string());
        value_list.push(value as f64 / 400.);
    });

    let chart = Chart::new()
        .title(Title::new().text("Population").left("center").top(20))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .visual_map(
            VisualMap::new()
                .show(false)
                .type_(VisualMapType::Continuous)
                .color(vec!["#0011dd", "#dd1100"])
                .min(0.)
                .max(1.),
        )
        .x_axis(
            Axis::new()
                .name("Generation")
                .name_text_style(TextStyle::new().font_size(18))
                .data(data_list),
        )
        .y_axis(
            Axis::new()
                .name("Choice")
                .name_text_style(TextStyle::new().font_size(18))
                .interval(0.1),
        )
        .series(Line::new().show_symbol(false).data(value_list));

    HtmlRenderer::new("Chart", 1200, 800)
        .theme(Theme::Dark)
        .save(&chart, "out.html")
        .expect("render html");
}
