#[cfg(test)]
mod email_tests {
    use crate::reader::email_reader::EmailReader;
    use crate::{Reader, Summary};

    #[test]
    fn test_basic_and_empty_emails() {
        let basic_email = "Subject: Party Time
From: Jake
To: David
Just finished my work. Time for a break!";
        let empty_email = "";

        let basic_reader = EmailReader::parse(basic_email.to_string()).unwrap();
        assert_eq!(basic_reader.msg_len(), 40);
        assert_eq!(
            basic_reader.summarize(),
            "Jake: Just finished my work. Time for a break!"
        );
        assert_eq!(basic_reader.get_info(), "Party Time\nFrom: Jake, To: David");

        let empty_result = EmailReader::parse(empty_email.to_string());
        assert!(empty_result.is_err());
        assert_eq!(
            empty_result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_malformed_emails() {
        let missing_fields = "Subject: Test Email
To: Recipient";
        let empty_subject = "Subject: \nFrom: sender\nTo: recipient\n";
        let empty_sender = "Subject: Test\nFrom: \nTo: recipient\n";
        let empty_recipient = "Subject: Test\nFrom: sender\nTo: ";

        for email in [missing_fields, empty_subject, empty_sender, empty_recipient].iter() {
            let result = EmailReader::parse(email.to_string());
            assert!(result.is_err());
            assert_eq!(
                result.as_ref().unwrap_err().kind(),
                std::io::ErrorKind::InvalidData
            );
            assert_eq!(
                result.as_ref().unwrap_err().to_string(),
                "File is not in the correct format"
            );
        }
    }

    fn generate_long_email() -> String {
        let subject = "Long Message Subject";
        let sender = "LongSenderName@example.com";
        let recipient = "LongRecipientName@example.org";

        // Generate a very long message, exceeding typical lengths
        let mut message = String::new();
        for _ in 0..1000 {
            message.push_str("This is a sentence in the very long message. ");
        }

        let email_str = format!(
            "Subject: {}\nFrom: {}\nTo: {}\n{}",
            subject, sender, recipient, message
        );

        email_str
    }

    #[test]
    fn test_summaries() {
        let short_email = "Subject: Meeting Reschedule
From: Executive Team
To: Department Heads
Please note the change in meeting schedule...";
        let long_email = generate_long_email();

        let short_reader = EmailReader::parse(short_email.to_string()).unwrap();
        let summary = short_reader.summarize();
        assert!(summary.starts_with("Executive Team: "));
        assert!(short_reader.get_info().contains("Meeting Reschedule"));

        let long_reader = EmailReader::parse(long_email).unwrap();
        let summary = long_reader.summarize();
        assert_eq!(summary.len(), 280);
        assert!(summary.contains("LongSenderName@example.com"));
        assert!(summary.contains("This is a sentence in the very long message. "));
    }

    #[test]
    fn test_summarize_extremely_long_message() {
        let subject = "Long Message Subject";
        let sender = "LongSenderName@example.com";
        let recipient = "LongRecipientName@example.org";

        // Generate a very long message, exceeding typical lengths
        let mut message = String::new();
        for _ in 0..1000 {
            message.push_str("This is a sentence in the very long message. ");
        }

        let email_str = format!(
            "Subject: {}\nFrom: {}\nTo: {}\n{}",
            subject, sender, recipient, message
        );

        let reader = EmailReader::parse(email_str.to_string()).unwrap();
        let summary = reader.summarize();
        assert_eq!(summary.len(), 280);

        assert!(summary.contains(sender));
        assert!(summary.contains("This is a sentence in the very long message. "));
    }
}

#[cfg(test)]
mod tweet_tests {
    use crate::reader::tweet_reader::TweetReader;
    use crate::{Reader, Summary};

    #[test]
    fn test_basic_and_empty_tweets() {
        let basic_tweet = "@1CoolDavid
\"Hey Rustaceans!
Excited to dive into some advanced topics today. Let's discuss ownership and lifetimes.
#RustLang\"";
        let empty_tweet = "";

        let basic_reader = TweetReader::parse(basic_tweet.to_string()).unwrap();
        assert_eq!(basic_reader.msg_len(), 113);
        assert_eq!(
            basic_reader.summarize(),
            "@1CoolDavid: Hey Rustaceans!
Excited to dive into some advanced topics today. Let's discuss ownership and lifetimes.
#RustLang"
        );
        assert_eq!(basic_reader.get_info(), "Tweet from @1CoolDavid");

        let empty_result = TweetReader::parse(empty_tweet.to_string());
        assert!(empty_result.is_err());
        assert_eq!(
            empty_result.as_ref().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_malformed_tweets() {
        let missing_fields = "@Connor";
        let missing_username = "\"Hello world!\"";
        let invalid_username = "InvalidUsername
\"Invalid tweet\"";
        let missing_initial_quote = "@user
This is content without a starting quote";
        let missing_ending_quote = "@user
\"This content is missing an ending quote";
        let invalid_tweet_type = "@user
\"Content\"
invalid";

        for tweet in [
            missing_fields,
            missing_username,
            invalid_username,
            missing_initial_quote,
            missing_ending_quote,
            invalid_tweet_type,
        ]
        .iter()
        {
            let result = TweetReader::parse(tweet.to_string());
            assert!(result.is_err());
            assert_eq!(
                result.as_ref().unwrap_err().kind(),
                std::io::ErrorKind::InvalidData
            );
            assert_eq!(
                result.as_ref().unwrap_err().to_string(),
                "File is not in the correct format"
            );
        }
    }

    #[test]
    fn test_retweets_and_replies() {
        let retweet = "@FerrisTheCrab
\"@1CoolDavid Absolutely, David! Ownership is key to Rust's safety.
Lifetimes help us manage references. Students, any questions so far?
#RustLearning\"
retweet";
        let reply = "@user
\"Content\"
reply";

        let retweet_reader = TweetReader::parse(retweet.to_string()).unwrap();
        assert_eq!(retweet_reader.msg_len(), 148);
        assert!(retweet_reader
            .summarize()
            .starts_with("@FerrisTheCrab: @1CoolDavid Absolutely,"));
        assert_eq!(
            retweet_reader.get_info(),
            "Tweet from @FerrisTheCrab (retweet)"
        );

        let reply_reader = TweetReader::parse(reply.to_string()).unwrap();
        assert_eq!(reply_reader.get_info(), "Tweet from @user (reply)");
    }

    #[test]
    fn test_edge_cases_and_long_content() {
        let empty_content = "@user
\"\"";
        let long_username = "VeryLongUserName12345";
        let special_characters_username = "user_with-underscores.123";
        let long_content = "@LongUserName
\"This is a very long tweet with a length of 1000 characters.
(Repeated many times to reach the desired length.)\"";

        let empty_content_reader = TweetReader::parse(empty_content.to_string()).unwrap();
        assert_eq!(empty_content_reader.msg_len(), 0);
        assert_eq!(empty_content_reader.get_info(), "Tweet from @user");
        assert_eq!(empty_content_reader.summarize(), "@user: ");

        let long_username_reader = TweetReader::parse(
            format!(
                "@{}
\"\"",
                long_username
            )
            .to_string(),
        )
        .unwrap();
        assert_eq!(
            long_username_reader.get_info(),
            format!("Tweet from @{}", long_username)
        );

        let special_characters_reader = TweetReader::parse(
            format!(
                "@{}
\"\"",
                special_characters_username
            )
            .to_string(),
        )
        .unwrap();
        assert_eq!(
            special_characters_reader.get_info(),
            format!("Tweet from @{}", special_characters_username)
        );

        let long_content_reader = TweetReader::parse(long_content.to_string()).unwrap();
        assert_eq!(long_content_reader.msg_len(), 110);
        assert!(long_content_reader.summarize().starts_with("@LongUserName"));
        assert!(!long_content_reader.summarize().contains(long_content));
    }
}
