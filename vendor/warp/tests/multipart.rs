#![deny(warnings)]
use bytes::BufMut;
use futures::{TryFutureExt, TryStreamExt};
use warp::{multipart, Filter};

#[tokio::test]
async fn form_fields() {
    let _ = pretty_env_logger::try_init();

    let route = multipart::form().and_then(|form: multipart::FormData| {
        async {
            // Collect the fields into (name, value): (String, Vec<u8>)
            let part: Result<Vec<(String, Vec<u8>)>, warp::Rejection> = form
                .and_then(|part| {
                    let name = part.name().to_string();
                    let value = part.stream().try_fold(Vec::new(), |mut vec, data| {
                        vec.put(data);
                        async move { Ok(vec) }
                    });
                    value.map_ok(move |vec| (name, vec))
                })
                .try_collect()
                .await
                .map_err(|e| {
                    panic!("multipart error: {:?}", e);
                });
            part
        }
    });

    let boundary = "--abcdef1234--";
    let body = format!(
        "\
         --{0}\r\n\
         content-disposition: form-data; name=\"foo\"\r\n\r\n\
         bar\r\n\
         --{0}--\r\n\
         ",
        boundary
    );

    let req = warp::test::request()
        .method("POST")
        .header("content-length", body.len())
        .header(
            "content-type",
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(body);

    let vec = req.filter(&route).await.unwrap();
    assert_eq!(&vec[0].0, "foo");
    assert_eq!(&vec[0].1, b"bar");
}
