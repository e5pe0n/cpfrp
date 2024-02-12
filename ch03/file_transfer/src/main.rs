use actix_web::Error;
use actix_web::{
    web::{self, Path},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::{ok, Future};
use futures::{TryFutureExt, TryStreamExt};
use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

fn flush_stdout() {
    io::stdout().flush().unwrap();
}

async fn delete_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Deleting file \"{}\" ... ", filename);
    flush_stdout();

    match std::fs::remove_file(&filename) {
        Ok(_) => {
            println!("Deleted file \"{}\"", filename);
            HttpResponse::Ok()
        }
        Err(error) => {
            println!("Failed to delete file \"{}\": {}", filename, error);
            HttpResponse::NotFound()
        }
    }
}

async fn download_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();

    fn read_file_contents(filename: &str) -> std::io::Result<String> {
        let mut contents = String::new();
        std::fs::File::open(filename)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    match read_file_contents(&filename) {
        Ok(contents) => {
            println!("Downloaded file \"{}\"", filename);
            HttpResponse::Ok().content_type("text/plain").body(contents)
        }
        Err(error) => {
            println!("Failed to read file \"{}\": {}", filename, error);
            HttpResponse::NotFound().finish()
        }
    }
}

async fn upload_specific_file(payload: web::Payload, info: Path<(String,)>) -> impl Responder {
    let filename = info.0.clone();

    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    payload
        .map_err(Error::from)
        .try_fold(web::BytesMut::new(), |mut body, chunk| async move {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(|contents| async move {
            let f = std::fs::File::create(&filename);
            if f.is_err() {
                println!("Failed to create file \"{}\"", filename);
                return Ok::<_, Error>(HttpResponse::NotFound().into());
            }

            if f.unwrap().write_all(&contents).is_err() {
                println!("Failed to write file \"{}\"", filename);
                return Ok(HttpResponse::NotFound().into());
            }

            println!("Uploaded file\"{}\"", filename);
            Ok(HttpResponse::Ok().finish())
        })
        .await
}

async fn upload_new_file(payload: web::Payload, info: Path<(String,)>) -> impl Responder {
    let filename_prefix = info.0.clone();
    print!("Uploading file \"{}*.txt\" ... ", filename_prefix);
    flush_stdout();

    payload
        .map_err(Error::from)
        .try_fold(web::BytesMut::new(), |mut body, chunk| async move {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(|contents| async move {
            let mut rng = rand::thread_rng();
            let mut attemps = 0;
            let mut file;
            let mut filename;
            const MAX_ATTEMPS: u32 = 100;

            loop {
                attemps += 1;
                if attemps > MAX_ATTEMPS {
                    println!(
                        "Failed to create new file with prefix \"{}\", after {} attemps.",
                        filename_prefix, MAX_ATTEMPS
                    );
                    return Ok(HttpResponse::NotFound().into());
                }

                filename = format!("{}{:03}.txt", filename_prefix, rng.gen_range(0..1000));

                file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&filename);

                if file.is_ok() {
                    break;
                }
            }

            if file.unwrap().write_all(&contents).is_err() {
                println!("Failed to write file \"{}\"", filename);
                return Ok(HttpResponse::NotFound().into());
            }

            println!("Uploaded file \"{}\"", filename);
            Ok(HttpResponse::Ok().content_type("text/plain").body(filename))
        })
        .await
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_addr = "127.0.0.1:8080";
    println!("Listening at address {} ...", server_addr);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_specific_file))
                    .route(web::post().to(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_addr)?
    .run()
    .await
}
