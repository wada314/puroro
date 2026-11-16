//! Opt-in comparison of handwritten `puroro-sample-generated` with the plugin
//! output for `fixtures/reference_schema`.
//!
//! Not compiled by `cargo test` or CI. Enable it with:
//!
//! ```text
//! cargo test -p puroro-codegen-tests --features sample-parity --lib sample_parity
//! ```
//!
//! A compile error or a failed assertion here is expected while
//! `sample-generated` is the design sketch and the plugin has not caught up.
//! Do not move this module onto the default test path to hide that.

use ::puroro::{BytesMut, Message, RepeatedStringMut, StringMut};

macro_rules! same {
    ($plugin:expr, $sample:expr) => {
        ::core::assert_eq!(
            $plugin,
            $sample,
            "{} vs {}",
            ::core::stringify!($plugin),
            ::core::stringify!($sample)
        );
    };
}

#[test]
fn field_numbers_match() {
    use crate::reference_schema::example::address as plugin_address;
    use crate::reference_schema::example::marker as plugin_marker;
    use crate::reference_schema::example::point as plugin_point;
    use crate::reference_schema::example::school as plugin_school;
    use crate::reference_schema::example::student as plugin_student;
    use crate::reference_schema::example::task as plugin_task;
    use ::puroro_sample_generated::address as sample_address;
    use ::puroro_sample_generated::marker as sample_marker;
    use ::puroro_sample_generated::point as sample_point;
    use ::puroro_sample_generated::school as sample_school;
    use ::puroro_sample_generated::student as sample_student;
    use ::puroro_sample_generated::task as sample_task;

    same!(plugin_address::FIELD_STREET, sample_address::FIELD_STREET);
    same!(plugin_address::FIELD_CITY, sample_address::FIELD_CITY);
    same!(
        plugin_address::FIELD_POSTAL_CODE,
        sample_address::FIELD_POSTAL_CODE
    );
    same!(
        plugin_address::FIELD_LATITUDE,
        sample_address::FIELD_LATITUDE
    );

    same!(plugin_point::FIELD_X, sample_point::FIELD_X);
    same!(plugin_point::FIELD_Y, sample_point::FIELD_Y);

    same!(plugin_task::FIELD_TITLE, sample_task::FIELD_TITLE);
    same!(plugin_task::FIELD_SCORE, sample_task::FIELD_SCORE);
    same!(
        plugin_task::FIELD_MAX_RETRIES,
        sample_task::FIELD_MAX_RETRIES
    );
    same!(plugin_task::FIELD_OWNER_ID, sample_task::FIELD_OWNER_ID);
    same!(plugin_task::FIELD_PAYLOAD, sample_task::FIELD_PAYLOAD);
    same!(plugin_task::FIELD_TAG_IDS, sample_task::FIELD_TAG_IDS);
    same!(plugin_task::FIELD_SCORES, sample_task::FIELD_SCORES);
    same!(plugin_task::FIELD_LABELS, sample_task::FIELD_LABELS);
    same!(plugin_task::FIELD_STATUS, sample_task::FIELD_STATUS);
    same!(plugin_task::FIELD_PRIORITY, sample_task::FIELD_PRIORITY);
    same!(plugin_task::FIELD_ASSIGNEE, sample_task::FIELD_ASSIGNEE);
    same!(
        plugin_task::FIELD_EMAIL_ADDRESS,
        sample_task::FIELD_EMAIL_ADDRESS
    );
    same!(
        plugin_task::FIELD_PHONE_NUMBER,
        sample_task::FIELD_PHONE_NUMBER
    );
    same!(plugin_task::FIELD_WEBHOOK_ID, sample_task::FIELD_WEBHOOK_ID);
    same!(plugin_task::FIELD_POSTAL, sample_task::FIELD_POSTAL);
    same!(plugin_task::FIELD_DONE, sample_task::FIELD_DONE);
    same!(plugin_task::FIELD_FLAG, sample_task::FIELD_FLAG);
    same!(plugin_task::FIELD_URGENT, sample_task::FIELD_URGENT);
    same!(plugin_task::FIELD_WATCHERS, sample_task::FIELD_WATCHERS);
    same!(plugin_task::FIELD_VOTES, sample_task::FIELD_VOTES);
    same!(plugin_task::FIELD_ATTRIBUTES, sample_task::FIELD_ATTRIBUTES);
    same!(plugin_task::FIELD_ORIGIN, sample_task::FIELD_ORIGIN);

    same!(plugin_student::FIELD_YEAR, sample_student::FIELD_YEAR);
    same!(
        plugin_student::FIELD_LOCATION,
        sample_student::FIELD_LOCATION
    );
    same!(plugin_student::FIELD_HOME, sample_student::FIELD_HOME);

    same!(plugin_school::FIELD_NAME, sample_school::FIELD_NAME);
    same!(plugin_school::FIELD_STUDENT, sample_school::FIELD_STUDENT);

    same!(plugin_marker::FIELD_N, sample_marker::FIELD_N);
}

#[test]
fn eager_bits_match() {
    use crate::reference_schema::example::address as plugin_address;
    use crate::reference_schema::example::school as plugin_school;
    use crate::reference_schema::example::student as plugin_student;
    use crate::reference_schema::example::task as plugin_task;
    use ::puroro_sample_generated::address as sample_address;
    use ::puroro_sample_generated::school as sample_school;
    use ::puroro_sample_generated::student as sample_student;
    use ::puroro_sample_generated::task as sample_task;

    same!(plugin_address::BIT_STREET, sample_address::BIT_STREET);
    same!(
        plugin_address::BIT_STREET_SSO,
        sample_address::BIT_STREET_SSO
    );
    same!(plugin_address::BIT_CITY, sample_address::BIT_CITY);
    same!(plugin_address::BIT_CITY_SSO, sample_address::BIT_CITY_SSO);
    same!(
        plugin_address::BIT_POSTAL_CODE,
        sample_address::BIT_POSTAL_CODE
    );
    same!(plugin_address::BIT_LATITUDE, sample_address::BIT_LATITUDE);

    same!(plugin_task::BIT_TITLE, sample_task::BIT_TITLE);
    same!(plugin_task::BIT_TITLE_SSO, sample_task::BIT_TITLE_SSO);
    same!(plugin_task::BIT_MAX_RETRIES, sample_task::BIT_MAX_RETRIES);
    same!(plugin_task::BIT_OWNER_ID, sample_task::BIT_OWNER_ID);
    same!(plugin_task::BIT_OWNER_ID_SSO, sample_task::BIT_OWNER_ID_SSO);
    same!(plugin_task::BIT_PAYLOAD, sample_task::BIT_PAYLOAD);
    same!(plugin_task::BIT_PAYLOAD_SSO, sample_task::BIT_PAYLOAD_SSO);
    same!(plugin_task::BIT_PRIORITY, sample_task::BIT_PRIORITY);
    same!(
        plugin_task::BIT_EMAIL_ADDRESS_SSO,
        sample_task::BIT_EMAIL_ADDRESS_SSO
    );
    same!(
        plugin_task::BIT_PHONE_NUMBER_SSO,
        sample_task::BIT_PHONE_NUMBER_SSO
    );
    same!(plugin_task::BIT_DONE_VALUE, sample_task::BIT_DONE_VALUE);
    same!(plugin_task::BIT_FLAG, sample_task::BIT_FLAG);
    same!(plugin_task::BIT_FLAG_VALUE, sample_task::BIT_FLAG_VALUE);
    same!(plugin_task::BIT_URGENT_VALUE, sample_task::BIT_URGENT_VALUE);
    same!(plugin_task::BIT_ORIGIN, sample_task::BIT_ORIGIN);

    same!(plugin_student::BIT_YEAR, sample_student::BIT_YEAR);
    same!(plugin_student::BIT_LOCATION, sample_student::BIT_LOCATION);
    same!(plugin_student::BIT_HOME, sample_student::BIT_HOME);

    same!(plugin_school::BIT_NAME, sample_school::BIT_NAME);
    same!(plugin_school::BIT_NAME_SSO, sample_school::BIT_NAME_SSO);
    same!(plugin_school::BIT_STUDENT, sample_school::BIT_STUDENT);
}

#[test]
fn task_lazy_kind_bits_match() {
    use crate::reference_schema::example::task as plugin_task;
    use ::puroro_sample_generated::task as sample_task;

    same!(
        plugin_task::BIT_TITLE_LAZY_KIND,
        sample_task::BIT_TITLE_LAZY_KIND
    );
    same!(
        plugin_task::BIT_OWNER_ID_LAZY_KIND,
        sample_task::BIT_OWNER_ID_LAZY_KIND
    );
    same!(
        plugin_task::BIT_PAYLOAD_LAZY_KIND,
        sample_task::BIT_PAYLOAD_LAZY_KIND
    );
    same!(
        plugin_task::BIT_EMAIL_ADDRESS_LAZY_KIND,
        sample_task::BIT_EMAIL_ADDRESS_LAZY_KIND
    );
    same!(
        plugin_task::BIT_PHONE_NUMBER_LAZY_KIND,
        sample_task::BIT_PHONE_NUMBER_LAZY_KIND
    );
}

/// Handwritten `Address` leaves bits 6 and 7 unused and starts the lazy-kind
/// indices at 8. The plugin packs those indices immediately after the eager
/// bits. The assertion stays so that gap is visible while the sample is the
/// design sketch.
#[test]
fn address_lazy_kind_bits_match() {
    use crate::reference_schema::example::address as plugin_address;
    use ::puroro_sample_generated::address as sample_address;

    same!(
        plugin_address::BIT_STREET_LAZY_KIND,
        sample_address::BIT_STREET_LAZY_KIND
    );
    same!(
        plugin_address::BIT_CITY_LAZY_KIND,
        sample_address::BIT_CITY_LAZY_KIND
    );
}

/// One populated `Task`. The two fills are the same sequence so a wire
/// difference is a real encode mismatch, not a different fixture.
macro_rules! encoded_task {
    ($root:path) => {{
        use $root::{Address, Point, Priority, Status, Task};

        let mut task = Task::new();
        task.title_mut().set("Write docs");
        *task.score_mut() = 42;
        *task.max_retries_mut() = 5;
        task.owner_id_mut().set("user-1");
        task.payload_mut().extend_from_slice(b"data");
        task.tag_ids_mut().push(10);
        task.tag_ids_mut().push(20);
        task.scores_mut().push(1);
        task.labels_mut().push().push_str("urgent");
        *task.status_mut() = Status::PENDING;
        *task.priority_mut() = Priority::HIGH;
        *task.done_mut() = true;
        *task.flag_mut() = false;
        task.email_address_mut().set("a@example.com");

        let mut assignee = Address::new();
        assignee.street_mut().set("1 Main St");
        assignee.city_mut().set("Tokyo");
        *assignee.postal_code_mut() = 1_000_001;
        *assignee.latitude_mut() = 35.6812;
        task.set_assignee(assignee);

        let mut watcher = Address::new();
        watcher.street_mut().set("2 Side Rd");
        watcher.city_mut().set("Osaka");
        *watcher.postal_code_mut() = 5_300_001;
        *watcher.latitude_mut() = 34.6937;
        task.watchers_mut().push(watcher);
        task.votes_mut().push(true);
        task.votes_mut().push(false);
        // Map entry order follows the map hasher, so attributes are not part
        // of this byte compare.

        let mut origin = Point::new();
        *origin.x_mut() = 1;
        *origin.y_mut() = 2;
        task.set_origin(origin);

        Message::encode_to_vec(&task)
    }};
}

#[test]
fn task_encode_matches() {
    let plugin = encoded_task!(crate::reference_schema::example);
    let sample = encoded_task!(::puroro_sample_generated);
    assert_eq!(plugin, sample);
}

#[test]
fn max_retries_default_matches() {
    use crate::reference_schema::example::Task as PluginTask;
    use ::puroro_sample_generated::Task as SampleTask;

    assert_eq!(
        PluginTask::new().max_retries().get(),
        SampleTask::new().max_retries().get()
    );
}
