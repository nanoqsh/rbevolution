use crate::model::Choice;

pub fn render(data: Vec<Choice>) {
    use charming::{
        component::{Axis, Legend, Title, VisualMap, VisualMapType},
        element::*,
        series::Line,
        theme::Theme,
        Chart, HtmlRenderer,
    };

    let mut data_list = vec![];
    let mut blue_list = vec![];
    let mut red_list = vec![];

    let mut counter = 0;
    data.iter().for_each(|value| {
        counter += 1;
        data_list.push(counter.to_string());
        blue_list.push(value.n_blu as i32);
        red_list.push(value.n_red as i32);
    });

    let chart = Chart::new()
        .background_color(Color::from("#3b303c"))
        .color(vec![Color::from("#f85d80"), Color::from("#318eb8")])
        .title(Title::new().text("Population").right(160).top(20))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new().top(20))
        .visual_map(
            VisualMap::new()
                .show(false)
                .type_(VisualMapType::Continuous)
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
                .name("Amount")
                .name_text_style(TextStyle::new().font_size(18)),
        )
        .series(
            Line::new()
                .name("Red")
                .emphasis(
                    Emphasis::new().item_style(ItemStyle::new().color(Color::from("#9f3b52"))),
                )
                .show_symbol(false)
                .stack("x")
                .line_style(LineStyle::new().width(0.))
                .area_style(AreaStyle::new().color(Color::LinearGradient {
                    x: 0.,
                    y: 0.,
                    x2: 0.,
                    y2: 1.,
                    color_stops: vec![ColorStop::new(0., "#f85d80"), ColorStop::new(1., "#9f3b52")],
                }))
                .data(red_list),
        )
        .series(
            Line::new()
                .name("Blue")
                .emphasis(
                    Emphasis::new().item_style(ItemStyle::new().color(Color::from("#25466b"))),
                )
                .show_symbol(false)
                .stack("x")
                .line_style(LineStyle::new().width(0.))
                .area_style(AreaStyle::new().color(Color::LinearGradient {
                    x: 0.,
                    y: 0.,
                    x2: 0.,
                    y2: 1.,
                    color_stops: vec![ColorStop::new(0., "#318eb8"), ColorStop::new(1., "#25466b")],
                }))
                .data(blue_list),
        );

    HtmlRenderer::new("Evolution", 1400, 800)
        .theme(Theme::Dark)
        .save(&chart, "out.html")
        .expect("render html");
}
