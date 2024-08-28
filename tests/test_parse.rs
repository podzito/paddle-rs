// MIT License

// Copyright (c) 2024 podzito

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use rstest::rstest;
use serde_json::Value;
use std::fs;

#[rstest]
#[case("subscription-annual-created.json")]
#[case("subscription-monthly-created.json")]
#[case("subscription-canceled.json")]
#[case("subscription-paused.json")]
#[case("subscription-resumed.json")]
#[case("subscription-updated.json")]
fn test_sum(#[case] filename: &str) {
    use paddle::model::notification::Notification;

    let event_json = fs::read_to_string(format!("tests/data/notifications/{}", filename)).unwrap();
    let notification: Notification = serde_json::from_str::<Value>(&event_json)
        .unwrap()
        .try_into()
        .unwrap();

    println!(
        "notification: {:?}",
        serde_json::to_string(&notification).unwrap()
    );

    let expected_value =
        fs::read_to_string(format!("tests/data/notifications/expected/{}", filename)).unwrap();
    let expected_notification: Notification =
        serde_json::from_str::<Notification>(&expected_value).unwrap();

    assert_eq!(notification, expected_notification);
}
