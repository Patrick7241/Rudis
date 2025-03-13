use fern::colors::{Color, ColoredLevelConfig};
use std::fs::OpenOptions;

pub fn setup_logger() -> Result<(), fern::InitError> {
    // 配置日志级别的颜色
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    // 创建一个根调度器
    fern::Dispatch::new()
        .format(move |out, message, record| {
            // 定义日志的输出格式
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors.color(record.level()),
                message = message
            ))
        })
        .level(log::LevelFilter::Debug)
        // 将日志输出到标准输出（控制台）
        .chain(std::io::stdout())
        // 将日志输出到文件
        .chain(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("app.log")?,
        )
        .apply()?;
    Ok(())
}
