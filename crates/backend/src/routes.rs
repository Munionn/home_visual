use crate::handlers::{
    auth_handler, automation_handler, device_handler, home_handler, robot_handler, room_handler,
    scene_handler,
};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth_handler::register))
                    .route("/login", web::post().to(auth_handler::login)),
            )
            .service(
                web::scope("/homes")
                    .route("", web::post().to(home_handler::create_home))
                    .route("", web::get().to(home_handler::list_homes))
                    .route("/{id}", web::get().to(home_handler::get_home))
                    .route("/{id}", web::put().to(home_handler::update_home))
                    .route("/{id}", web::delete().to(home_handler::delete_home)),
            )
            .service(
                web::scope("/rooms")
                    .route("", web::post().to(room_handler::create_room))
                    .route("", web::get().to(room_handler::list_rooms))
                    .route("/{id}", web::get().to(room_handler::get_room))
                    .route("/{id}", web::put().to(room_handler::update_room))
                    .route("/{id}", web::delete().to(room_handler::delete_room)),
            )
            .service(
                web::scope("/devices")
                    .route("", web::post().to(device_handler::create_device))
                    .route("", web::get().to(device_handler::list_devices))
                    .route("/{id}", web::get().to(device_handler::get_device))
                    .route("/{id}", web::put().to(device_handler::update_device))
                    .route("/{id}", web::delete().to(device_handler::delete_device))
                    .route(
                        "/{id}/command",
                        web::post().to(device_handler::send_command),
                    ),
            )
            .service(
                web::scope("/robot")
                    .route("/tasks", web::post().to(robot_handler::create_task))
                    .route("/tasks", web::get().to(robot_handler::list_tasks))
                    .route("/tasks/{id}", web::get().to(robot_handler::get_task_status))
                    .route(
                        "/tasks/{id}/cancel",
                        web::post().to(robot_handler::cancel_task),
                    ),
            )
            .service(
                web::scope("/scenes")
                    .route("", web::post().to(scene_handler::create_scene))
                    .route("", web::get().to(scene_handler::list_scenes))
                    .route("/{id}", web::get().to(scene_handler::get_scene))
                    .route("/{id}", web::put().to(scene_handler::update_scene))
                    .route("/{id}", web::delete().to(scene_handler::delete_scene))
                    .route(
                        "/{id}/activate",
                        web::post().to(scene_handler::activate_scene),
                    ),
            )
            .service(
                web::scope("/automations")
                    .route("", web::post().to(automation_handler::create_automation))
                    .route("", web::get().to(automation_handler::list_automations))
                    .route(
                        "/{id}/{home_id}",
                        web::get().to(automation_handler::get_automation),
                    )
                    .route(
                        "/{id}",
                        web::put().to(automation_handler::update_automation),
                    )
                    .route(
                        "/{id}",
                        web::delete().to(automation_handler::delete_automation),
                    )
                    .route(
                        "/{id}/toggle",
                        web::post().to(automation_handler::toggle_automation),
                    ),
            ),
    );
}
