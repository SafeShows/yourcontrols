use discord_game_sdk::{Activity, CreateFlags, Discord};
use spin_sleep::sleep;
use std::time::{Duration, SystemTime};

pub struct DiscordRPC<'a> {
  client: Discord<'a, ()>,
  state: String,
  start_time: i64,
  is_default_activity_set: bool,
}

impl<'a> DiscordRPC<'a> {
  pub fn new() -> Self {
    let client: Discord<()> = Discord::with_create_flags(776052285235003392, CreateFlags::Default)
      .expect("Failed to connect to discord!");
    let start_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap_or_default()
      .as_secs_f64() as i64;
    Self {
      client: client,
      state: "Main Menu".to_string(),
      start_time: start_time,
      is_default_activity_set: false,
    }
  }

  pub fn set_default_activity(&mut self) {
    let mut activity = Activity::empty();
    activity
      .with_details(&self.state)
      .with_start_time(self.start_time);

    self.is_default_activity_set = true;
    self
      .client
      .update_activity(&activity, |_discord: &Discord<()>, result| {
        if let Err(error) = result {
          return eprintln!("failed to update activity: {}", error);
        }
      });
  }

  pub fn do_callback(&mut self) -> () {
    if !self.is_default_activity_set {
      self.set_default_activity();
    }
    self.client.run_callbacks().ok();
  }
}
