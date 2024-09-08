//use mongodb::bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MessageProperties {
    can_be_deleted_only_for_self: bool, //v
    can_be_deleted_for_all_users: bool, //v
    can_be_edited: bool,                //v
    can_be_forwarded: bool,             //v
    can_be_paid: bool,
    can_be_pinned: bool,
    can_be_replied: bool,
    can_be_replied_in_another_chat: bool,
    can_be_saved: bool, //v
    can_be_shared_in_story: bool,
    can_edit_scheduling_state: bool,
    can_get_added_reactions: bool,
    can_get_embedding_code: bool,
    can_get_link: bool,
    can_get_media_timestamp_links: bool, //v
    can_get_message_thread: bool,        //v
    can_get_read_date: bool,
    can_get_statistics: bool, //v
    can_get_viewers: bool,    //v
    can_recognize_speech: bool,
    can_report_chat: bool,
    can_report_reactions: bool,
    can_report_supergroup_spam: bool,
    can_set_fact_check: bool,
    need_show_statistics: bool,
}

impl MessageProperties {
    pub fn can_be_deleted_only_for_self(&self) -> bool {
        self.can_be_deleted_only_for_self
    }
    pub fn can_be_deleted_for_all_users(&self) -> bool {
        self.can_be_deleted_for_all_users
    }
    pub fn can_be_edited(&self) -> bool {
        self.can_be_edited
    }
    pub fn can_be_forwarded(&self) -> bool {
        self.can_be_forwarded
    }
    pub fn can_be_paid(&self) -> bool {
        self.can_be_paid
    }
    pub fn can_be_pinned(&self) -> bool {
        self.can_be_pinned
    }
    pub fn can_be_replied(&self) -> bool {
        self.can_be_replied
    }
    pub fn can_be_replied_in_another_chat(&self) -> bool {
        self.can_be_replied_in_another_chat
    }
    pub fn can_be_saved(&self) -> bool {
        self.can_be_saved
    }
    pub fn can_be_shared_in_story(&self) -> bool {
        self.can_be_shared_in_story
    }
    pub fn can_edit_scheduling_state(&self) -> bool {
        self.can_edit_scheduling_state
    }
    pub fn can_get_added_reactions(&self) -> bool {
        self.can_get_added_reactions
    }
    pub fn can_get_embedding_code(&self) -> bool {
        self.can_get_embedding_code
    }
    pub fn can_get_link(&self) -> bool {
        self.can_get_link
    }
    pub fn can_get_media_timestamp_links(&self) -> bool {
        self.can_get_media_timestamp_links
    }
    pub fn can_get_message_thread(&self) -> bool {
        self.can_get_message_thread
    }
    pub fn can_get_read_date(&self) -> bool {
        self.can_get_read_date
    }
    pub fn can_get_statistics(&self) -> bool {
        self.can_get_statistics
    }
    pub fn can_get_viewers(&self) -> bool {
        self.can_get_viewers
    }
    pub fn can_recognize_speech(&self) -> bool {
        self.can_recognize_speech
    }
    pub fn can_report_chat(&self) -> bool {
        self.can_report_chat
    }
    pub fn can_report_reactions(&self) -> bool {
        self.can_report_reactions
    }
    pub fn can_report_supergroup_spam(&self) -> bool {
        self.can_report_supergroup_spam
    }
    pub fn can_set_fact_check(&self) -> bool {
        self.can_set_fact_check
    }
    pub fn need_show_statistics(&self) -> bool {
        self.need_show_statistics
    }
}
