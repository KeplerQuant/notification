use notification::{manager::NotifierManager, telegram::Telegram};

#[tokio::test]
async fn test_send_message() {
    let mut tg = Telegram::new("6008334649:AAEqViXWNM1QOe87LUFifK6_A9rwaDW1YN4");
    tg.add_recipient(5902201639);

    let mut notifier_manager = NotifierManager::new();
    notifier_manager.add_notifier(Box::new(tg));

    let res = notifier_manager.notify("subject", "body").await;
    assert!(res.is_ok())
}
