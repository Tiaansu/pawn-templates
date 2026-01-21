mod internals;
mod natives;
mod plugin;

use crate::plugin::PawnTemplates;
use samp::initialize_plugin;

initialize_plugin!(
    natives: [
        PawnTemplates::create_template,
        PawnTemplates::render_template,
        PawnTemplates::delete_template
    ],
    {
        samp::plugin::enable_process_tick();
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let log_file = fern::log_file("PawnTemplates.log").expect("Cannot create log file!");

        let trace_level = fern::Dispatch::new()
            .level(log::LevelFilter::Trace)
            .chain(log_file);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[PawnTemplates] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .chain(trace_level)
            .apply();

        PawnTemplates {
            pool: Vec::new()
        }
    }
);
