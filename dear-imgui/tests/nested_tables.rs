use dear_imgui_rs::{Condition, Context, TableFlags};

#[test]
fn nested_tables_can_be_created_while_parent_is_open() {
    let mut context = Context::create();
    context.io_mut().set_display_size([800.0, 600.0]);
    context.io_mut().set_delta_time(1.0 / 60.0);
    context
        .set_ini_filename::<std::path::PathBuf>(None)
        .unwrap();
    context
        .font_atlas()
        .try_claim_legacy_renderer()
        .unwrap()
        .build();
    let mut rendered_parents = 0;
    let mut rendered_children = 0;
    for _ in 0..3 {
        let ui = context.frame();
        ui.window("Nested table scope regression")
            .size([800.0, 600.0], Condition::Always)
            .build(|| {
                ui.table("parent")
                    .flags(TableFlags::SCROLL_Y)
                    .column("Name")
                    .done()
                    .build(|ui| {
                        rendered_parents += 1;
                        // Allocate distinct tables while the parent token is alive,
                        // exercising growth of ImGui's table pool.
                        for i in 0..32 {
                            ui.table_next_row();
                            ui.table_next_column();
                            ui.table(format!("child##{i}"))
                                .column("Value")
                                .done()
                                .build(|ui| {
                                    rendered_children += 1;
                                    ui.table_next_row();
                                    ui.table_next_column();
                                    ui.text("instance");
                                });
                        }
                    });
            });
        drop(context.render_legacy());
    }
    assert_eq!(rendered_parents, 3);
    assert!(rendered_children > 0);
}
