use std;
use steamworks;

pub fn upload_item_content(
    ugc: &steamworks::UGC, appid: u32,
    published_id: steamworks::PublishedFileId,
    content: &str, preview: &str
) {
    // uploading the content of the workshop item
    // this process uses a builder pattern to set properties of the item
    // mandatory properties are:
    // - title
    // - description
    // - preview_path
    // - content_path
    // - visibility
    // after setting the properties, call .submit() to start uploading the item
    // this function is unique in that it returns a handle to the upload, which can be used to
    // monitor the progress of the upload and needs a closure to be called when the upload is done
    // in this example, the watch handle is ignored for simplicity
    //
    // notes:
    // - once an upload is started, it cannot be cancelled!
    // - content_path is the path to a folder which houses the content you wish to upload
    let _upload_handle = ugc
        .start_item_update(steamworks::AppId(appid), published_id)
        .content_path(std::path::Path::new(content))
        .preview_path(std::path::Path::new(preview))
        .title("Item title")
        .description("Item description")
        .tags(Vec::<String>::new(), false)
        .visibility(steamworks::PublishedFileVisibility::Public)
        .submit(Some("My changenotes"), |upload_result| {
            // handle the result
            match upload_result {
                Ok((published_id, needs_to_agree_to_terms)) => {
                    if needs_to_agree_to_terms {
                        // as stated in the create_item function, if the user needs to agree to the terms of use,
                        // the upload did NOT succeed, despite the result being Ok
                        println!(
                            "You need to agree to the terms of use before you can upload any files"
                        );
                    } else {
                        // this is the definite indicator that an item was uploaded successfully
                        // the watch handle is NOT an accurate indicator whether the upload is done
                        // the progress on the other hand IS accurate and can simply be used to monitor the upload
                        println!("Uploaded item with id {:?}", published_id);
                    }
                }
                Err(e) => {
                    // the upload failed
                    // the exact reason can be found in the error type
                    println!("Error uploading item: {:?}", e);
                }
            }
        });
}