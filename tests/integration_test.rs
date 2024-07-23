use notification::{telegram::Telegram, Notification};

#[tokio::test]
async fn test_send_message() {
    let mut tg = Telegram::new("6008334649:AAEqViXWNM1QOe87LUFifK6_A9rwaDW1YN4");
    tg.add_recipient(5902201639);

    let mut notification = Notification::new();
    notification.add_notifier(tg);

    let res = notification.notify("subject", "body").await;
    assert!(res.is_ok())
}
