use clap::{arg, Arg, ArgAction, ArgMatches, Command};
use std::{fs, path};
use walkdir;

use crate::tasks;
use crate::types::{
    EdgesDetectionParams, FourierTransformParams, Method, Params, ProjectionParams,
};

fn get_params(matches: &ArgMatches) -> Option<Params> {
    let method = match matches.get_one::<Method>("METHOD") {
        Some(md) => md.to_owned(),
        None => Method::Default,
    };

    let mut check_projection_params = false;
    let mut check_edges_detection_params = false;
    let mut check_fourier_transform_params = false;

    match method {
        Method::Default => {
            check_projection_params = true;
            check_edges_detection_params = true;
        }
        Method::ProjectionOnly => check_projection_params = true,
        Method::EdgesDetectionOnly => check_edges_detection_params = true,
        Method::FourierTransformOnly => check_fourier_transform_params = true,
    }

    let mut params = Params::new();

    if check_projection_params {
        let projection_max_angle: u16 = match matches.get_one::<String>("pma") {
            Some(str) => str.parse().unwrap(),
            None => return None,
        };
        let projection_angle_step: f64 = match matches.get_one::<String>("pas") {
            Some(str) => str.parse().unwrap(),
            None => return None,
        };
        let projection_max_resolution: Vec<&str> = match matches.get_one::<String>("pmr") {
            Some(r) => r,
            None => "0x0",
        }
        .split("x")
        .collect();
        if projection_max_resolution.len() != 2 {
            return None;
        }
        let (projection_max_width, projection_max_height) = {
            let width = match i32::from_str_radix(projection_max_resolution[0], 10) {
                Ok(val) => val,
                Err(_) => return None,
            };
            let height = match i32::from_str_radix(projection_max_resolution[1], 10) {
                Ok(val) => val,
                Err(_) => return None,
            };
            (width, height)
        };
        params.set_projection_params(ProjectionParams {
            projection_max_angle,
            projection_angle_step,
            projection_max_width,
            projection_max_height,
        });
    }

    if check_edges_detection_params {
        let min_line_length: f64 = match matches.get_one::<String>("elg") {
            Some(str) => str.parse().unwrap(),
            None => return None,
        };
        let max_line_gap: f64 = match matches.get_one::<String>("egp") {
            Some(str) => str.parse().unwrap(),
            None => return None,
        };
        params.set_edges_params(EdgesDetectionParams {
            min_line_length,
            max_line_gap,
        });
    }

    if check_fourier_transform_params {
        let min_line_length: f64 = match matches.get_one::<String>("flg") {
            Some(str) => str.parse().unwrap(),
            None => return None,
        };
        let max_line_gap: f64 = match matches.get_one::<String>("fgp") {
            Some(str) => str.parse().unwrap(),
            None => return None,
        };
        params.set_fourier_params(FourierTransformParams {
            min_line_length,
            max_line_gap,
        });
    }

    Some(params)
}

pub fn bootstrap() {
    let app = Command::new("oic")
        .arg(arg!(-i <INPUT> "输入文件地址"))
        .arg(arg!(-o <OUTPUT> "输出文件地址"))
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .action(ArgAction::SetTrue),
        )
        .arg(arg!(-m <METHOD> "使用算法"))
        .arg(arg!(--pma <pma> "投影法最大偏转角"))
        .arg(arg!(--pas <PROJECTION_PARAMS_ANGLE_STEP> "投影法搜索步进值"))
        .arg(arg!(--pmr <PROJECTION_PARAMS_MAX_RESOLUTION> "投影法最大像素"))
        .arg(arg!(--elg <EDGES_PARAMS_MIN_LINE_LENGTH> "边缘检测法最小长度"))
        .arg(arg!(--egp <EDGES_PARAMS_MAX_LINE_GAP> "边缘检测法最大间断"))
        .arg(arg!(--flg <FOURIER_PARAMS_MIN_LINE_LENGTH> "傅里叶变换法边缘检测最小长度"))
        .arg(arg!(--fgp <FOURIER_PARAMS_MAX_LINE_GAP> "傅里叶变换法边缘检测最大间断"))
        .arg_required_else_help(true);

    let matches = app.get_matches();

    let params = get_params(&matches);

    if params.is_none() {
        return;
    }

    let params = params.unwrap();

    let input = matches
        .get_one::<String>("INPUT")
        .expect("请输入待处理文件路径");

    let output = matches
        .get_one::<String>("OUTPUT")
        .expect("请输出输出文件路径");

    let with_dir = matches.get_flag("dir");

    if with_dir {
        if fs::read_dir(input).is_err() {
            println!("文件夹 `{}` 不存在！", input);
            std::process::exit(-1);
        }

        if !path::Path::new(output).is_dir() {
            println!("输出路径必须是文件夹！");
            std::process::exit(-1);
        }

        fs::create_dir_all(output).unwrap();
    } else {
        if fs::File::open(input).is_err() {
            println!("文件 `{}` 不存在！", input);
            std::process::exit(-1);
        }

        if path::Path::new(output).is_dir() {
            fs::create_dir_all(output).unwrap();
        }
    }

    let method = match matches.get_one::<Method>("METHOD") {
        Some(md) => md.to_owned(),
        None => Method::Default,
    };

    if with_dir {
        for entry in walkdir::WalkDir::new(input) {
            let this_entry = entry.unwrap();
            if !this_entry.metadata().unwrap().is_file() {
                continue;
            }

            let filepath = this_entry.path().display();
            let input_file_path = filepath.to_string();
            let file_name = String::from(this_entry.file_name().to_str().unwrap());

            tasks::new_task(
                method.clone(),
                params.clone(),
                input_file_path,
                String::from(path::Path::new(output).join(&file_name).to_str().unwrap()),
            );
        }
    } else {
        let output: String = if path::Path::new(output).is_dir() {
            let file_name = path::Path::new(input).file_name().unwrap();
            String::from(path::Path::new(output).join(&file_name).to_str().unwrap())
        } else {
            String::from(output)
        };
        tasks::new_task(method, params, String::from(input), output);
    }

    loop {}
}
