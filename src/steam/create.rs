use steamworks;

pub fn create_item(ugc: &steamworks::UGC, appid: u32) {
    // creating a new workshop item
    // make sure you change the appid to the specified game
    ugc.create_item(
        steamworks::AppId(appid),
        steamworks::FileType::Community,
        |create_result| {
            // handle the result
            match create_result {
                Ok((published_id, needs_to_agree_to_terms)) => {
                    // if the user needs to agree to the terms of use, they will need to do that before you can upload any files
                    // in any case, make sure you save the published_id somewhere, like a manifest file.
                    // it is needed for all further calls
                    if needs_to_agree_to_terms {
                        println!(
                            "You need to agree to the terms of use before you can upload any files"
                        );
                    } else {
                        println!("Published item with id {:?}", published_id);
                    }
                }
                Err(e) => {
                    // an error occurred, usually because the app is not authorized to create items
                    // or the user is banned from the community
                    println!("Error creating workshop item: {:?}", e);
                }
            }
        },
    );
}