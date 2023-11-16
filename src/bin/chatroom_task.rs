//! Module that encapsulates everything needed to create a new chatroom task by the main broker task

pub async fn chatroom_broker() -> Result<(), ServerError>